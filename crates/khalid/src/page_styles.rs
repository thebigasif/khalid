//! Each page's own stylesheet: the rules of the site's stylesheet that the
//! page's HTML can use, so a post never downloads the rules of the home
//! page, and the reverse (ADR "each page links the rules it uses").
//!
//! A selector is left out only when it names, outside any brackets, a
//! class that no element of the page has. Such a selector can match
//! nothing on the page, so leaving it out changes nothing the reader sees.
//! Every other selector stays: one with no class, one that names a class
//! only inside `:is()`, `:not()` or `:has()`, and one with an escaped
//! character. At-rules other than `@media`, `@supports`, `@container` and
//! `@layer` stay as they are, such as `@font-face` and `@keyframes`.
//!
//! The classes the reader script adds to the page count as on every page,
//! because the script adds them after the page has loaded. Rules handed
//! over to be kept, such as a site's own from `theme/site.css`, stay on
//! every page whole, since a script of the site's may add their classes.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};

use khalid_render::Asset;

/// The at-rules whose rules are filtered one by one. Any other at-rule is
/// kept whole.
const GROUPS: [&str; 4] = ["media", "supports", "container", "layer"];

/// A site's stylesheet read into rules, ready to give each page the rules
/// its HTML can use. Pages that use the same classes share one sheet, made
/// once.
pub(crate) struct PageStyles {
    items: Vec<Item>,
    /// Every class a selector needs, with its place in a [`Classes`] set.
    classes: BTreeMap<String, usize>,
    /// The classes every page counts as having: the reader script's.
    always: Classes,
    /// The sheets made so far, with the classes they were made for. A
    /// site has a few, so a list is enough.
    made: Mutex<Vec<(Classes, Arc<PageSheet>)>>,
}

/// One page's stylesheet: its text and the hashed name it is served at,
/// such as `site.3f2a9c1e.css`.
pub(crate) struct PageSheet {
    name: String,
    text: String,
}

impl PageSheet {
    /// The hashed file name, such as `site.3f2a9c1e.css`. The same text
    /// always gets the same name.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// The stylesheet's text.
    pub(crate) fn text(&self) -> &str {
        &self.text
    }
}

/// One piece of a stylesheet.
enum Item {
    /// Text kept as it is written: an at-rule other than a group, or a
    /// rule this reader does not take apart.
    Kept(String),
    /// A style rule: each selector with the classes it needs, and its
    /// declarations.
    Rule {
        selectors: Vec<Selector>,
        body: String,
    },
    /// `@media` and the other groups: their prelude and the items inside.
    Group { prelude: String, items: Vec<Item> },
}

/// One selector of a rule and the classes an element must have for it to
/// match.
struct Selector {
    text: String,
    needs: Vec<usize>,
}

/// A set of classes, one bit each, in the order of [`PageStyles::classes`].
#[derive(Clone, PartialEq, Eq)]
struct Classes(Vec<u64>);

impl Classes {
    /// An empty set with room for `count` classes.
    fn with_room(count: usize) -> Self {
        Self(vec![0; count.div_ceil(64)])
    }

    fn insert(&mut self, bit: usize) {
        if let Some(word) = self.0.get_mut(bit / 64) {
            *word |= 1 << (bit % 64);
        }
    }

    fn contains(&self, bit: usize) -> bool {
        self.0
            .get(bit / 64)
            .is_some_and(|word| word & (1 << (bit % 64)) != 0)
    }
}

impl fmt::Debug for PageStyles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageStyles")
            .field("classes", &self.classes.len())
            .finish_non_exhaustive()
    }
}

impl PageStyles {
    /// Reads `css`, a stylesheet as `stylesheet_text_with` builds it, whose
    /// rules each page gets only when it can use them, followed by `kept`,
    /// rules every page gets whole, such as a site's own. The classes named
    /// in the strings of `script`, the reader script, count as on every page.
    pub(crate) fn new(css: &str, kept: &str, script: &str) -> Self {
        let mut classes = BTreeMap::new();
        let mut items = read(css, &mut classes);
        let kept = kept.trim();
        if !kept.is_empty() {
            items.push(Item::Kept(kept.to_owned()));
        }
        let mut always = Classes::with_room(classes.len());
        for word in script_words(script) {
            if let Some(&bit) = classes.get(word) {
                always.insert(bit);
            }
        }
        Self {
            items,
            classes,
            always,
            made: Mutex::default(),
        }
    }

    /// The stylesheet for the page `html`: every rule that can match one of
    /// its elements, and no other.
    pub(crate) fn for_page(&self, html: &str) -> Arc<PageSheet> {
        let mut used = self.always.clone();
        each_class(html, |class| {
            if let Some(&bit) = self.classes.get(class) {
                used.insert(bit);
            }
        });
        let mut made = self.made.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((_, sheet)) = made.iter().find(|(classes, _)| *classes == used) {
            return Arc::clone(sheet);
        }
        let mut text = String::new();
        write_items(&self.items, &used, &mut text);
        let name = Asset::hashed_name("site", "css", text.as_bytes());
        let sheet = Arc::new(PageSheet { name, text });
        made.push((used, Arc::clone(&sheet)));
        sheet
    }
}

impl PageStyles {
    /// Every sheet made so far, in the order they were made.
    pub(crate) fn made(&self) -> Vec<Arc<PageSheet>> {
        let made = self.made.lock().unwrap_or_else(PoisonError::into_inner);
        made.iter().map(|(_, sheet)| Arc::clone(sheet)).collect()
    }
}

/// Reads `css` into items, noting each class a selector needs in `classes`.
fn read(css: &str, classes: &mut BTreeMap<String, usize>) -> Vec<Item> {
    let mut items = Vec::new();
    let mut rest = css.trim_start();
    while !rest.is_empty() {
        let Some((at, stop)) = find_top(rest, &['{', ';']) else {
            items.push(Item::Kept(rest.to_owned()));
            break;
        };
        if stop == ';' {
            let (statement, after) = rest.split_at(at.saturating_add(1));
            items.push(Item::Kept(statement.to_owned()));
            rest = after.trim_start();
            continue;
        }
        let Some(close) = closing_brace(rest, at) else {
            items.push(Item::Kept(rest.to_owned()));
            break;
        };
        let (whole, after) = rest.split_at(close.saturating_add(1));
        let prelude = whole.get(..at).unwrap_or_default().trim();
        let body = whole.get(at.saturating_add(1)..close).unwrap_or_default();
        items.push(item(prelude, body, whole, classes));
        rest = after.trim_start();
    }
    items
}

/// One block read from `whole`: its `prelude` and the `body` between its
/// braces.
fn item(prelude: &str, body: &str, whole: &str, classes: &mut BTreeMap<String, usize>) -> Item {
    if let Some(name) = prelude.strip_prefix('@') {
        let name = name
            .split(|c: char| !is_name_char(c))
            .next()
            .unwrap_or_default();
        return if GROUPS.iter().any(|group| group.eq_ignore_ascii_case(name)) {
            Item::Group {
                prelude: prelude.to_owned(),
                items: read(body, classes),
            }
        } else {
            Item::Kept(whole.to_owned())
        };
    }
    // A rule with rules nested inside is kept whole.
    if find_top(body, &['{']).is_some() {
        return Item::Kept(whole.to_owned());
    }
    let selectors = selectors(prelude, classes);
    Item::Rule {
        selectors,
        body: body.to_owned(),
    }
}

/// Writes the items a page with the classes `used` keeps.
fn write_items(items: &[Item], used: &Classes, out: &mut String) {
    for item in items {
        match item {
            Item::Kept(text) => {
                out.push_str(text);
                out.push('\n');
            }
            Item::Rule { selectors, body } => {
                let start = out.len();
                let kept = selectors
                    .iter()
                    .filter(|selector| selector.needs.iter().all(|&bit| used.contains(bit)));
                for selector in kept {
                    if out.len() > start {
                        out.push_str(", ");
                    }
                    out.push_str(&selector.text);
                }
                if out.len() > start {
                    out.push_str(" {");
                    out.push_str(body);
                    out.push_str("}\n");
                }
            }
            Item::Group { prelude, items } => {
                let start = out.len();
                out.push_str(prelude);
                out.push_str(" {\n");
                let inside = out.len();
                write_items(items, used, out);
                if out.len() == inside {
                    out.truncate(start);
                } else {
                    out.push_str("}\n");
                }
            }
        }
    }
}

/// The selectors of `prelude`, split at the commas outside brackets and
/// strings, each with the classes it names outside any brackets, noted in
/// `classes`. An element must have each of them for the selector to match.
/// A selector with an escaped character needs none, so it is always kept.
fn selectors(prelude: &str, classes: &mut BTreeMap<String, usize>) -> Vec<Selector> {
    let mut found = Vec::new();
    let mut needs = Vec::new();
    let mut scan = Scan::default();
    let mut start = 0;
    let end = (prelude.len(), ',');
    for (at, c) in prelude.char_indices().chain(std::iter::once(end)) {
        let top = at == prelude.len() || scan.step(c) == Step::Top;
        if top && c == ',' {
            let text = prelude.get(start..at).unwrap_or_default().trim();
            if text.contains('\\') {
                needs.clear();
            }
            if !text.is_empty() {
                let needs = std::mem::take(&mut needs);
                found.push(Selector {
                    text: text.to_owned(),
                    needs,
                });
            }
            needs.clear();
            start = at.saturating_add(1);
        } else if top && c == '.' {
            let name = prelude.get(at.saturating_add(1)..).unwrap_or_default();
            let name = name
                .split(|c: char| !is_name_char(c))
                .next()
                .unwrap_or_default();
            if name.starts_with(|c: char| !c.is_ascii_digit()) {
                let next = classes.len();
                needs.push(*classes.entry(name.to_owned()).or_insert(next));
            }
        }
    }
    found
}

/// Calls `take` with every class named in a `class` attribute of `html`,
/// and some words that are not, such as those of a `data-class`
/// attribute. A word too many only keeps a rule that matches nothing; a
/// class missed would drop a rule the page needs, so the search is wide.
fn each_class(html: &str, mut take: impl FnMut(&str)) {
    let mut rest = html;
    while let Some(at) = find_class(rest) {
        rest = rest.get(at.saturating_add(5)..).unwrap_or_default();
        let Some(value) = rest.trim_start().strip_prefix('=') else {
            continue;
        };
        let value = value.trim_start();
        let quote = value.chars().next().filter(|&c| c == '"' || c == '\'');
        let value = value
            .get(quote.map_or(0, char::len_utf8)..)
            .unwrap_or_default();
        let end = value
            .find(|c: char| match quote {
                Some(quote) => c == quote,
                None => c.is_ascii_whitespace() || c == '>',
            })
            .unwrap_or(value.len());
        value
            .get(..end)
            .unwrap_or_default()
            .split_ascii_whitespace()
            .for_each(&mut take);
    }
}

/// Where `class`, in any case, first starts in `text`.
fn find_class(text: &str) -> Option<usize> {
    text.as_bytes()
        .windows(5)
        .position(|window| window.eq_ignore_ascii_case(b"class"))
}

/// Every word inside a string of the script `js`, such as `lightbox` in
/// `dialog.className = "lightbox"`, leaving out comment lines. The script
/// names each class it adds in a string, so these hold all of them, and
/// some words that are not classes.
fn script_words(js: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    for line in js.lines() {
        if quote.is_none() && line.trim_start().starts_with("//") {
            continue;
        }
        let mut start = None;
        for (at, c) in line.char_indices() {
            let inside = quote.is_some() && !escaped;
            if inside && is_name_char(c) {
                start.get_or_insert(at);
                continue;
            }
            if let Some(from) = start.take() {
                words.extend(line.get(from..at));
            }
            if escaped {
                escaped = false;
            } else if c == '\\' && quote.is_some() {
                escaped = true;
            } else if quote == Some(c) {
                quote = None;
            } else if quote.is_none() && matches!(c, '"' | '\'' | '`') {
                quote = Some(c);
            }
        }
        if let Some(from) = start {
            words.extend(line.get(from..));
        }
    }
    words
}

/// Whether `c` may be part of a class name in a selector.
fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_' || !c.is_ascii()
}

/// Where the first of the characters `stops` sits in `text` outside
/// brackets and strings, and which character it is.
fn find_top(text: &str, stops: &[char]) -> Option<(usize, char)> {
    let mut scan = Scan::default();
    text.char_indices()
        .find(|&(_, c)| scan.step(c) == Step::Top && stops.contains(&c))
}

/// Where the brace that closes the one at `open` sits in `text`, skipping
/// strings and nested braces.
fn closing_brace(text: &str, open: usize) -> Option<usize> {
    let mut scan = Scan::default();
    let mut depth = 0_usize;
    for (at, c) in text.get(open..)?.char_indices() {
        if scan.step(c) == Step::InString {
            continue;
        }
        match c {
            '{' => depth = depth.saturating_add(1),
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(at.saturating_add(open));
                }
            }
            _ => {}
        }
    }
    None
}

/// Where a character sits: in a string, inside brackets, or at the top.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    InString,
    Nested,
    Top,
}

/// Tracks strings, escapes and bracket depth one character at a time.
#[derive(Default)]
struct Scan {
    quote: Option<char>,
    escaped: bool,
    depth: usize,
}

impl Scan {
    /// Takes `c` and says where it sits. An opening bracket sits outside
    /// itself and a closing one inside, so both count as nested.
    // Kept out of line: four readers call it, and a copy in each costs the
    // binary budget.
    #[inline(never)]
    fn step(&mut self, c: char) -> Step {
        if self.escaped {
            self.escaped = false;
            return Step::InString;
        }
        if c == '\\' {
            self.escaped = true;
            return Step::InString;
        }
        if let Some(quote) = self.quote {
            if c == quote {
                self.quote = None;
            }
            return Step::InString;
        }
        match c {
            '"' | '\'' => {
                self.quote = Some(c);
                return Step::InString;
            }
            '(' | '[' => {
                self.depth = self.depth.saturating_add(1);
                return Step::Nested;
            }
            ')' | ']' => {
                self.depth = self.depth.saturating_sub(1);
                return Step::Nested;
            }
            _ => {}
        }
        if self.depth == 0 {
            Step::Top
        } else {
            Step::Nested
        }
    }
}

#[cfg(test)]
#[path = "page_styles_tests.rs"]
mod tests;

//! The files every page links, built once: the stylesheet from the theme,
//! and the reader script from the bytes in the binary. `khalid build`
//! writes them and `khalid serve` answers them.

use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::{fs, io};

use khalid_core::{Config, SitePath, UserMessage};
use khalid_render::{
    Asset, Feature, Features, ScriptFile, ScriptTag, Tokens, reader_script, reader_script_file,
    stylesheet_text_with, stylesheet_with,
};
use khalid_server::{MediaType, Page, PagePath};
use khalid_theme::Look;

use crate::build_pages::LanguagePages;
use crate::page_styles::{PageSheet, PageStyles};
use crate::{build_message, look};

/// The folder, under the mount and in the output folder, that holds them.
pub(crate) const FOLDER: &str = "assets/";

/// The reader features the reader script improves. With all of them off,
/// pages link no script at all.
const SCRIPTED: [Feature; 5] = [
    Feature::HeadingLinks,
    Feature::EnlargeImages,
    Feature::SearchShortcut,
    Feature::TableOfContents,
    Feature::LinkPreviews,
];

/// The reader script's name, hash and bytes, made once per process because
/// they never change while the binary runs. Only `khalid build` makes its
/// compressed copies; `khalid serve` compresses after it starts answering,
/// so the script never delays the first page.
static READER: LazyLock<ScriptFile> = LazyLock::new(reader_script_file);

/// The file in a site folder that holds the site's own CSS rules.
const SITE_CSS: &str = "theme/site.css";

/// The folder in a site folder that holds its self-hosted font files.
const FONTS: &str = "theme/fonts/";

/// The site's stylesheet: the values in `theme/tokens.toml` over the
/// look's defaults, as CSS custom properties, then the shared rules, then
/// the look's own rules and the site's own.
pub(crate) fn site_stylesheet(site: &Path, look: &Look) -> Result<Asset, UserMessage> {
    let tokens = look::tokens(site, look)?;
    stylesheet_with(&tokens, &rules(site, look)?).map_err(|error| build_message::render(&error))
}

/// The look's own rules, then the site's own from `theme/site.css` when the
/// site has that file, so a rule of the site's wins over the look's.
///
/// # Errors
///
/// The message for a file that leads outside the site folder or cannot be
/// read.
pub(crate) fn rules(site: &Path, look: &Look) -> Result<String, UserMessage> {
    Ok([look.pack().stylesheet(), &own_rules(site)?].join("\n"))
}

/// The site's own rules from `theme/site.css`, empty when the site has no
/// such file.
///
/// # Errors
///
/// The message for a file that leads outside the site folder or cannot be
/// read.
pub(crate) fn own_rules(site: &Path) -> Result<String, UserMessage> {
    let path = SitePath::within(site, SITE_CSS)
        .and_then(|path| path.resolve())
        .map_err(|error| build_message::site_css(SITE_CSS, &error))?;
    match fs::read_to_string(&path) {
        Ok(text) => Ok(text),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(build_message::site_css(SITE_CSS, &error)),
    }
}

/// One self-hosted font the stylesheet names: its file name in
/// `theme/fonts/`, which is also its name under `<mount>assets/fonts/`,
/// and its bytes.
#[derive(Debug)]
pub(crate) struct FontFile {
    pub(crate) name: String,
    pub(crate) bytes: Vec<u8>,
}

/// The font files `tokens` lists, read from `theme/fonts/` in the tokens'
/// order, each once. Only a plain `*.woff2` name the tokens list is read,
/// so nothing else in the folder, or outside it, is ever served. A file
/// that cannot be read is left out with a warning, and the stylesheet's
/// next font stands in (G20).
pub(crate) fn fonts(site: &Path, tokens: &Tokens) -> Vec<FontFile> {
    let mut found: Vec<FontFile> = Vec::new();
    for listed in &tokens.fonts.files {
        let name = listed.path.as_str();
        if !is_plain_woff2(name) || found.iter().any(|font| font.name == name) {
            continue;
        }
        let read = font_path(site, name).and_then(|path| fs::read(path).ok());
        let Some(bytes) = read else {
            tracing::warn!(file = name, "a listed font file could not be read");
            continue;
        };
        found.push(FontFile {
            name: name.to_owned(),
            bytes,
        });
    }
    found
}

/// Whether `name` is a plain `*.woff2` file name, with no folder in it,
/// the only kind of font file the site serves.
fn is_plain_woff2(name: &str) -> bool {
    let plain = !name.contains(['/', '\\']) && !name.starts_with('.');
    plain
        && Path::new(name)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("woff2"))
}

/// Where the font file `name` is on disk, inside `site`'s `theme/fonts/`.
fn font_path(site: &Path, name: &str) -> Option<PathBuf> {
    SitePath::within(site, &format!("{FONTS}{name}"))
        .ok()
        .and_then(|path| path.resolve().ok())
}

/// Names the font file that draws body text in every language's pages,
/// which preload it, when the site's `tokens` list one it serves (G20). A
/// file that is not there is not named, so no page asks for it for
/// nothing. A headless site has no tokens and no pages to name it in.
pub(crate) fn preload_body_font(
    site: &Path,
    config: &Config,
    tokens: Option<&Tokens>,
    languages: &mut [LanguagePages],
) {
    let Some(file) = tokens.and_then(|tokens| tokens.fonts.body_file()) else {
        return;
    };
    let name = file.path.as_str();
    let served = is_plain_woff2(name) && font_path(site, name).is_some_and(|path| path.is_file());
    if !served {
        return;
    }
    let href = format!("{}{FOLDER}fonts/{name}", config.mount());
    for language in languages {
        language.site.body_font = Some(href.clone());
    }
}

/// The fonts as `khalid serve` holds them, at `<mount>assets/fonts/<file>`,
/// where the stylesheet's `@font-face` rules point. Each is kept for a
/// year: `theme sync` names a file by its face, and a new face gets a new
/// name.
pub(crate) fn font_pages(config: &Config, fonts: Vec<FontFile>) -> Result<Vec<Page>, UserMessage> {
    fonts
        .into_iter()
        .map(|font| {
            let href = format!("{}{FOLDER}fonts/{}", config.mount(), font.name);
            let path = PagePath::try_new(&href).map_err(|_| build_message::address(&href))?;
            Ok(Page::immutable(path, MediaType::Woff2, font.bytes))
        })
        .collect()
}

/// The reader script's name, integrity value and bytes.
pub(crate) fn reader() -> &'static ScriptFile {
    &READER
}

/// The reader script with its compressed copies, for `khalid build`.
pub(crate) fn reader_with_copies() -> Result<Asset, UserMessage> {
    reader_script().map_err(|error| build_message::render(&error))
}

/// The tag pages with `features` carry for the reader script, or `None`
/// when no feature on uses it.
pub(crate) fn script_tag(config: &Config, features: &Features) -> Option<ScriptTag> {
    let used = SCRIPTED.iter().any(|&feature| features.contains(feature));
    let folder = format!("{}{FOLDER}", config.mount());
    used.then(|| ScriptTag::of_file(reader(), &folder))
}

/// The address pages link `asset` at, such as `/blog/assets/site.3f2a9c1e.css`.
pub(crate) fn href(config: &Config, asset: &Asset) -> String {
    format!("{}{FOLDER}{}", config.mount(), asset.name())
}

/// The stylesheet as `khalid serve` holds it from `tokens` and the look's
/// and site's [`rules`], cached for a year because its name changes with
/// its content, and each page's own stylesheet made from it. It is the
/// file [`site_stylesheet`] builds, at the same address, made without the
/// compressed copies: the server compresses its pages after it answers,
/// and Brotli's best setting would cost the start tens of megabytes.
pub(crate) fn stylesheets(
    config: &Config,
    tokens: &Tokens,
    (look_rules, own): (&str, &str),
) -> Result<(Page, PageStylesheets), UserMessage> {
    let css = stylesheet_text_with(tokens, &[look_rules, own].join("\n"));
    let name = Asset::hashed_name("site", "css", css.as_bytes());
    let href = format!("{}{FOLDER}{name}", config.mount());
    let path = PagePath::try_new(&href).map_err(|_| build_message::address(&href))?;
    let sheets = PageStylesheets::new(config, &href, (tokens, look_rules), &css);
    Ok((Page::immutable(path, MediaType::Css, css), sheets))
}

/// Each page's own stylesheet: the rules of the site's stylesheet that its
/// HTML can use (ADR "each page links the rules it uses"). A page links it
/// in place of the whole stylesheet, so no page downloads the rules of
/// other pages. Pages made outside the site's render, such as a comment's
/// refusal page, still link the whole stylesheet.
#[derive(Debug)]
pub(crate) struct PageStylesheets {
    styles: PageStyles,
    /// The tag the templates write for the whole stylesheet.
    whole: String,
    /// Where the sheets live, such as `/blog/assets/`.
    folder: String,
}

impl PageStylesheets {
    /// The sheets made from `whole`, the whole stylesheet built from
    /// `tokens` and `look_rules` and then the site's own rules, which pages
    /// link at `href`. Only Khalid's rules are left out where a page cannot
    /// use them: the site's own stay on every page, since a script of the
    /// site's may add the classes they name.
    pub(crate) fn new(
        config: &Config,
        href: &str,
        (tokens, look_rules): (&Tokens, &str),
        whole: &str,
    ) -> Self {
        let ours = stylesheet_text_with(tokens, look_rules);
        let script = String::from_utf8_lossy(reader().body);
        let styles = match whole.strip_prefix(ours.as_str()) {
            Some(own) => PageStyles::new(&ours, own, &script),
            None => PageStyles::new("", whole, &script),
        };
        Self {
            styles,
            whole: link(href),
            folder: format!("{}{FOLDER}", config.mount()),
        }
    }

    /// `html` with its own stylesheet linked in place of the whole one, and
    /// that stylesheet's address. A page that does not link the whole
    /// stylesheet is left as it is, with no address.
    pub(crate) fn fit(&self, html: String) -> (String, Option<String>) {
        if !html.contains(&self.whole) {
            return (html, None);
        }
        let href = self.href(&self.styles.for_page(&html));
        let html = html.replace(&self.whole, &link(&href));
        (html, Some(href))
    }

    /// Every page's own stylesheet made so far, each once.
    pub(crate) fn made(&self) -> Vec<Arc<PageSheet>> {
        self.styles.made()
    }

    /// The address a page links `sheet` at, such as
    /// `/blog/assets/site.3f2a9c1e.css`.
    pub(crate) fn href(&self, sheet: &PageSheet) -> String {
        format!("{}{}", self.folder, sheet.name())
    }

    /// `sheet` as `khalid serve` holds it, cached for a year like the whole
    /// stylesheet.
    pub(crate) fn page(&self, sheet: &PageSheet) -> Result<Page, UserMessage> {
        let href = self.href(sheet);
        let path = PagePath::try_new(&href).map_err(|_| build_message::address(&href))?;
        Ok(Page::immutable(
            path,
            MediaType::Css,
            sheet.text().to_owned(),
        ))
    }
}

/// The tag `base.html` writes for the stylesheet at `href`.
fn link(href: &str) -> String {
    format!("<link rel=\"stylesheet\" href=\"{href}\">")
}

/// The reader script as `khalid serve` holds it, cached for a year like
/// the stylesheet.
pub(crate) fn script_page(config: &Config) -> Result<Page, UserMessage> {
    let script = reader();
    let href = format!("{}{FOLDER}{}", config.mount(), script.name);
    let path = PagePath::try_new(&href).map_err(|_| build_message::address(&href))?;
    Ok(Page::immutable(
        path,
        MediaType::JavaScript,
        script.body.to_vec(),
    ))
}

#[cfg(test)]
mod tests {
    use khalid_core::Environment;
    use khalid_render::{stylesheet, stylesheet_text};

    use super::*;

    #[test]
    fn the_served_stylesheet_is_the_built_one_at_the_same_address() {
        let text = "name = \"a\"\naddress = \"https://www.example.com\"\n\
                    default_language = \"en\"\nlanguages = [\"en\"]\n";
        let config =
            Config::from_toml(Path::new("site.toml"), text, &Environment::empty()).unwrap();
        let tokens = Tokens::from_toml("[colors]\nlink = \"#b3261e\"").unwrap();
        let built = stylesheet(&tokens).unwrap();
        let (page, _) = stylesheets(&config, &tokens, ("", "")).unwrap();
        assert_eq!(
            page.body().unwrap().as_ref(),
            stylesheet_text(&tokens).as_bytes()
        );
        assert_eq!(page.path().as_str(), href(&config, &built));
        let folder = format!("{}{FOLDER}site.", config.mount());
        assert!(page.path().as_str().starts_with(&folder));
        assert_eq!(page.body().unwrap().as_ref(), built.body());
    }
}

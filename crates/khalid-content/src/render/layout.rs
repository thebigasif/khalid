//! The HTML of the layout blocks. Each reads in full with scripts off.

use std::num::NonZeroU8;

use super::{Renderer, attribute, inside};
use crate::{Block, Column, ProsConsBlock};

impl Renderer<'_> {
    /// Writes a pull quote as a `<figure>`: the words in a `<blockquote>`,
    /// and who said them as its caption.
    pub(super) fn pull_quote(&mut self, block: &Block, by: Option<&str>) {
        self.html
            .push_str("<figure class=\"pullquote\">\n<blockquote>\n");
        self.markdown(&inside(&block.source));
        self.html.push_str("</blockquote>\n");
        if let Some(by) = by {
            self.push(&["<figcaption>", &attribute(by), "</figcaption>\n"]);
        }
        self.html.push_str("</figure>\n");
    }

    /// Writes pros and cons: a titled list for each side that has a line.
    /// The titles say which list is which, so the look never relies on
    /// colour or an icon alone.
    pub(super) fn pros_cons(&mut self, block: &ProsConsBlock) {
        self.html.push_str("<div class=\"proscons\">\n");
        let sides = [
            ("pros", &block.pros_title, &block.pros),
            ("cons", &block.cons_title, &block.cons),
        ];
        for (class, title, items) in sides {
            if items.is_empty() {
                continue;
            }
            self.push(&[
                "<div class=\"",
                class,
                "\">\n<p class=\"proscons-title\">",
                &attribute(title),
                "</p>\n",
            ]);
            let list = items
                .iter()
                .map(|item| ["- ", item].concat())
                .collect::<Vec<_>>()
                .join("\n");
            self.markdown(&list);
            self.html.push_str("</div>\n");
        }
        self.html.push_str("</div>\n");
    }

    /// Writes columns: one card per column, each with its title and its
    /// markdown, in a grid that stacks on a narrow screen.
    pub(super) fn columns(&mut self, columns: &[Column]) {
        self.html.push_str("<div class=\"columns\">\n");
        for column in columns {
            self.html.push_str("<div class=\"column\">\n");
            if let Some(title) = &column.title {
                self.push(&["<p class=\"column-title\">", &attribute(title), "</p>\n"]);
            }
            self.markdown(&column.text);
            self.html.push_str("</div>\n");
        }
        self.html.push_str("</div>\n");
    }

    /// Writes a folding table: the first `fold` rows as a table, then the
    /// rest in a second table with the same header inside a `<details>`, so
    /// every row reads with scripts off. A table no longer than `fold`
    /// stays one plain table.
    pub(super) fn folding_table(&mut self, block: &Block, fold: NonZeroU8, more: Option<&str>) {
        let source = inside(&block.source);
        let lines: Vec<&str> = source
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        let (head, rows) = lines.split_at_checked(2).unwrap_or_default();
        let Some((shown, folded)) = rows
            .split_at_checked(usize::from(fold.get()))
            .filter(|(_, folded)| !folded.is_empty())
        else {
            self.markdown(&source);
            return;
        };
        self.html.push_str("<div class=\"table-fold\">\n");
        self.markdown(&[head, shown].concat().join("\n"));
        let summary = more.map_or_else(
            || match folded.len() {
                1 => "Show 1 more row".to_owned(),
                count => format!("Show {count} more rows"),
            },
            str::to_owned,
        );
        self.push(&["<details>\n<summary>", &attribute(&summary), "</summary>\n"]);
        // The header shows twice but is read once.
        let header_words: usize = head.first().map_or(0, |header| {
            header
                .split('|')
                .map(|cell| cell.split_whitespace().count())
                .sum()
        });
        let before = self.facts.words;
        self.markdown(&[head, folded].concat().join("\n"));
        self.facts.words = self.facts.words.saturating_sub(header_words).max(before);
        self.html.push_str("</details>\n</div>\n");
    }
}

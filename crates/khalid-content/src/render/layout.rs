//! The HTML of the layout blocks. Each reads in full with scripts off.

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
}

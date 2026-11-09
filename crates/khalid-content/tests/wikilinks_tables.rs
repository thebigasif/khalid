//! Wikilinks in a table cell: the `|` of `[[slug|text]]` belongs to the
//! link, not to the table, the way GitHub reads `\|` in a cell.

use khalid_content::{
    ContentError, ContentTree, WikiTarget, Wikilinks, check_bodies, read_content, render,
    render_linked,
};
use khalid_core::{Config, Environment, Locale, Slug, SlugMode};
use khalid_routes::Routes;

const DEMO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/demo-site");

#[cfg(test)]
fn en() -> Locale {
    Locale::try_new("en").unwrap()
}

#[cfg(test)]
fn targets() -> Wikilinks {
    let mut links = Wikilinks::new();
    links.insert(
        &Slug::try_new("topical-authority").unwrap(),
        WikiTarget::new("/blog/glossary/topical-authority/", "Topical authority"),
    );
    links
}

#[cfg(test)]
fn html(markdown: &str) -> String {
    render_linked(markdown, &en(), SlugMode::Ascii, &targets())
        .unwrap()
        .html
}

/// A two-column table whose second body cell is `cell`.
#[cfg(test)]
fn table(cell: &str) -> String {
    format!("| Term | Read |\n| --- | --- |\n| Authority | {cell} |\n")
}

const LABELLED: &str = "<td><a href=\"/blog/glossary/topical-authority/\">what it means</a></td>";

#[test]
fn a_labelled_wikilink_in_a_cell_is_a_link_with_its_label() {
    let html = html(&table("[[topical-authority|what it means]]"));
    assert!(html.contains(LABELLED), "{html}");
    assert_eq!(html.matches("<th>").count(), 2, "{html}");
    assert_eq!(html.matches("<td>").count(), 2, "{html}");
    assert!(!html.contains("[["), "{html}");
}

#[test]
fn an_escaped_pipe_gives_the_same_link() {
    let escaped = html(&table("[[topical-authority\\|what it means]]"));
    assert_eq!(escaped, html(&table("[[topical-authority|what it means]]")));
}

#[test]
fn an_unlabelled_wikilink_in_a_cell_is_named_after_its_target() {
    let html = html(&table("[[topical-authority]]"));
    assert!(
        html.contains(
            "<td><a href=\"/blog/glossary/topical-authority/\">Topical authority</a></td>"
        ),
        "{html}"
    );
}

#[test]
fn a_labelled_wikilink_in_the_header_row_keeps_the_table() {
    let html = html("| [[topical-authority|Authority]] | Read |\n| --- | --- |\n| a | b |\n");
    assert!(
        html.contains("<th><a href=\"/blog/glossary/topical-authority/\">Authority</a></th>"),
        "{html}"
    );
    assert_eq!(html.matches("<th>").count(), 2, "{html}");
}

#[test]
fn a_plain_pipe_still_splits_the_row() {
    let html = html("| a | b | c |\n| --- | --- | --- |\n| 1 | 2 | 3 |\n");
    assert_eq!(html.matches("<td>").count(), 3, "{html}");
}

#[test]
fn a_wikilink_in_code_or_outside_a_table_is_left_as_it_was() {
    let html = html(&table("`[[a|b]]`"));
    assert!(html.contains("<td><code>[[a|b]]</code></td>"), "{html}");
    let html = self::html("a | b and [[topical-authority|what]].\n");
    assert_eq!(
        html,
        "<p>a | b and <a href=\"/blog/glossary/topical-authority/\">what</a>.</p>\n"
    );
}

#[test]
fn a_wikilink_in_a_cell_is_an_internal_link_for_the_linter() {
    let rendered = render_linked(
        &table("[[topical-authority|what it means]]"),
        &en(),
        SlugMode::Ascii,
        &targets(),
    )
    .unwrap();
    let link = rendered.links.first().unwrap();
    assert_eq!(link.href, "/blog/glossary/topical-authority/");
    assert_eq!(link.text, "what it means");
    assert!(link.internal);
}

#[test]
fn a_table_in_a_render_without_targets_keeps_its_columns() {
    let html = render(
        &table("[[topical-authority|what it means]]"),
        &en(),
        SlugMode::Ascii,
    )
    .unwrap()
    .html;
    assert_eq!(html.matches("<td>").count(), 2, "{html}");
}

#[test]
fn a_broken_wikilink_in_a_cell_is_still_refused() {
    let dir = tempfile::tempdir().unwrap();
    let mut site = dir.path().to_path_buf();
    site.push("site.toml");
    std::fs::copy(format!("{DEMO}/site.toml"), &site).unwrap();
    let mut posts = dir.path().to_path_buf();
    posts.extend(["content", "en", "posts"]);
    std::fs::create_dir_all(&posts).unwrap();
    posts.push("cells.md");
    let body = table("[[no-such-post|gone]]");
    let text = format!("---\ntitle: Cells\nslug: cells\ndate: 2026-09-01\n---\n\n{body}");
    std::fs::write(&posts, text).unwrap();
    let config = Config::load(dir.path(), &Environment::empty()).unwrap();
    let content = read_content(&ContentTree::walk(dir.path()).unwrap()).unwrap();
    let error = check_bodies(&content, &config, &Routes::default()).unwrap_err();
    let ContentError::Wikilink { target, line, .. } = error else {
        panic!("{error:?}");
    };
    assert_eq!(target, "no-such-post");
    assert_eq!(line, 9);
}

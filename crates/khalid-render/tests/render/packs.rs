//! A template pack's files and options reach the pages: its files replace
//! the shared ones of the same name, a site's own theme still wins, and
//! templates read the options as `template.options` (templates spec 4.1).

use std::collections::BTreeMap;
use std::fs;

use khalid_render::{PageBody, PageKind, RenderError, RenderPack, Renderer, TemplateOption};

use crate::common::{demo_site, meta};

const PACK_PAGE: &str = "{% extends \"base.html\" %}{% block main %}\
<h1>{{ t.not_found_title }}</h1><p class=\"pack\">{{ template.name }} says \
{{ template.options.greeting }}{% if template.options.banner %} loudly{% endif %}</p>\
{% endblock %}";

fn pack(templates: Vec<(&'static str, &'static str)>) -> RenderPack<'static> {
    let mut options = BTreeMap::new();
    options.insert(
        "greeting".to_owned(),
        TemplateOption::Word("hello".to_owned()),
    );
    options.insert("banner".to_owned(), TemplateOption::Switch(true));
    RenderPack {
        name: "trial",
        templates,
        options,
    }
}

fn not_found(renderer: &Renderer) -> String {
    let meta = meta("Not found", "/blog/missing/", PageKind::Website);
    renderer
        .render(&demo_site("en"), &meta, &PageBody::NotFound)
        .unwrap()
}

#[test]
fn a_pack_file_replaces_the_shared_one_and_reads_its_options() {
    let site = tempfile::tempdir().unwrap();
    let renderer =
        Renderer::with_pack(site.path(), &pack(vec![("not_found.html", PACK_PAGE)])).unwrap();
    let html = not_found(&renderer);
    assert!(
        html.contains("<p class=\"pack\">trial says hello loudly</p>"),
        "{html}"
    );
}

#[test]
fn classic_renders_as_before() {
    let site = tempfile::tempdir().unwrap();
    let classic = Renderer::with_pack(site.path(), &RenderPack::classic()).unwrap();
    let before = Renderer::with_theme(site.path()).unwrap();
    assert_eq!(not_found(&classic), not_found(&before));
}

#[test]
fn a_site_theme_file_wins_over_the_pack() {
    let site = tempfile::tempdir().unwrap();
    let mut folder = site.path().to_path_buf();
    folder.push("theme/templates");
    fs::create_dir_all(&folder).unwrap();
    folder.push("not_found.html");
    let own = "{% extends \"base.html\" %}{% block main %}<h1>Mine</h1>{% endblock %}";
    fs::write(&folder, own).unwrap();
    let renderer =
        Renderer::with_pack(site.path(), &pack(vec![("not_found.html", PACK_PAGE)])).unwrap();
    let html = not_found(&renderer);
    assert!(html.contains("<h1>Mine</h1>"), "{html}");
    assert!(!html.contains("class=\"pack\""), "{html}");
}

#[test]
fn a_pack_file_that_replaces_nothing_is_refused() {
    let site = tempfile::tempdir().unwrap();
    let error = Renderer::with_pack(site.path(), &pack(vec![("sidebar.html", "x")])).unwrap_err();
    assert!(
        matches!(&error, RenderError::UnknownTemplate { name } if name == "sidebar.html"),
        "{error:?}"
    );
}

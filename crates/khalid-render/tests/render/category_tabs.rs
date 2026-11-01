//! The category tabs on the home page: "All" and a link to each category
//! that has posts, switched on by a template's `category_tabs` part, and a
//! pack's own post template that a post asks for with `template:`.

use khalid_core::TemplateName;
use khalid_render::{Link, PageBody, PageKind, RenderPack, TemplateOption};

use crate::common::{demo_site, meta};
use crate::html_check::assert_valid_page;
use crate::listing::{first_page, home, main_of};
use crate::parts::{pack_with, renderer};

fn render(pack: &RenderPack<'_>) -> String {
    let meta = meta("Field Notes on SEO", "/blog/", PageKind::Website);
    let mut front = home(first_page());
    front.categories = vec![
        Link {
            name: "On-page SEO".to_owned(),
            href: "/blog/category/on-page-seo/".to_owned(),
        },
        Link {
            name: "Technical SEO".to_owned(),
            href: "/blog/category/technical-seo/".to_owned(),
        },
    ];
    let page = renderer(pack)
        .render(&demo_site("en"), &meta, &PageBody::Home(front))
        .unwrap();
    assert_valid_page(&page);
    main_of(&page).to_owned()
}

#[test]
fn the_tabs_list_all_posts_then_each_category() {
    let on = pack_with(&[("category_tabs", TemplateOption::Switch(true))]);
    let html = render(&on);
    let tabs = "<nav class=\"category-tabs\" aria-label=\"Categories\">\n<ul>\n\
<li><a href=\"/blog/\" aria-current=\"page\">All</a></li>\n\
<li><a href=\"/blog/category/on-page-seo/\">On-page SEO</a></li>\n\
<li><a href=\"/blog/category/technical-seo/\">Technical SEO</a></li>\n\
</ul>\n</nav>";
    assert!(html.contains(tabs), "{html}");
}

#[test]
fn classic_shows_no_tabs_even_with_categories() {
    let html = render(&RenderPack::classic());
    assert!(!html.contains("category-tabs"), "{html}");
}

#[test]
fn a_pack_may_add_a_post_template_a_post_asks_for() {
    let mut pack = pack_with(&[]);
    pack.templates = vec![(
        "post-reference.html",
        "{% extends \"base.html\" %}{% block main %}<h1>Reference</h1>{% endblock %}",
    )];
    let renderer = renderer(&pack);
    let reference = TemplateName::try_new("reference").unwrap();
    assert!(renderer.has_template(&reference));
}

#[test]
fn a_pack_file_that_is_neither_shared_nor_a_post_template_is_refused() {
    let mut pack = pack_with(&[]);
    pack.templates = vec![("sidebar.html", "<aside></aside>")];
    let site = tempfile::tempdir().unwrap();
    let error = khalid_render::Renderer::with_pack(site.path(), &pack).unwrap_err();
    assert!(error.to_string().contains("sidebar.html"), "{error}");
}

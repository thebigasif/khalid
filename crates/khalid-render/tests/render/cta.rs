//! The site's call to action from `[template.cta]`: a band after a post or
//! on the front page, and a button in the header, each switched on by the
//! template's options (templates spec 4.2).

use khalid_render::{RenderCta, Renderer, TemplateOption};

use crate::common::demo_site;
use crate::html_check::assert_valid_page;
use crate::post::part;
use crate::slots::{home_on, post_on, with_pack, word};

pub(crate) fn trial_cta() -> RenderCta {
    RenderCta {
        label: "Start a free trial".to_owned(),
        href: "https://www.example.com/signup/".to_owned(),
        note: Some("No card needed.".to_owned()),
    }
}

#[test]
fn the_band_shows_the_note_and_the_button_where_the_template_says() {
    let renderer = with_pack(&[("cta_band", word("post"))], Some(trial_cta()));
    let page = post_on(&renderer, &demo_site("en"));
    assert_valid_page(&page);
    insta::assert_snapshot!(part(&page, "<div class=\"cta-band\">", "</div>"));
    assert!(!home_on(&renderer, &demo_site("en")).contains("cta-band"));

    let both = with_pack(&[("cta_band", word("both"))], Some(trial_cta()));
    let home = home_on(&both, &demo_site("en"));
    assert_valid_page(&home);
    assert!(home.contains("<div class=\"cta-band\">"), "{home}");
}

#[test]
fn the_header_button_links_to_the_call_to_action() {
    let options = [("cta_button", TemplateOption::Switch(true))];
    let page = post_on(&with_pack(&options, Some(trial_cta())), &demo_site("en"));
    assert_valid_page(&page);
    let header = part(&page, "<header class=\"site-header\">", "</header>");
    let button = "<a class=\"button header-button\" href=\"https://www.example.com/signup/\">Start a free trial</a>";
    assert!(header.contains(button), "{header}");
}

#[test]
fn nothing_shows_without_the_table_or_the_options() {
    let on = [
        ("cta_band", word("both")),
        ("cta_button", TemplateOption::Switch(true)),
    ];
    let page = post_on(&with_pack(&on, None), &demo_site("en"));
    assert!(!page.contains("cta-band") && !page.contains("header-button"));
    let classic = Renderer::new().unwrap();
    let page = post_on(&classic, &demo_site("en"));
    assert!(!page.contains("cta-band") && !page.contains("header-button"));
}

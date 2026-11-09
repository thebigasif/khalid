//! Title and description templates with variables (task 1.13, KH-SEO-1).

use chrono::{NaiveDate, TimeZone, Utc};
use khalid_core::{AbsoluteUrl, Locale};
use khalid_seo::{
    MetaTemplate, PageKind, PageNumber, SeoError, SeoPage, SeoSite, TemplateWords, UrlPath,
};

#[cfg(test)]
fn demo_site() -> SeoSite {
    let mut site = SeoSite::new(
        "Field Notes on SEO",
        AbsoluteUrl::parse("https://www.example.com").unwrap(),
        Locale::try_new("en").unwrap(),
    );
    site.tagline = String::from("Plain advice on search for small teams");
    site
}

#[cfg(test)]
fn post(title: &str) -> SeoPage {
    let mut page = SeoPage::new(
        PageKind::Post,
        Locale::try_new("en").unwrap(),
        UrlPath::try_new("/why-internal-links-matter/").unwrap(),
        title,
    );
    page.excerpt =
        String::from("Internal links help readers and search engines find the rest of your site.");
    page.category = Some(String::from("On-page SEO"));
    page
}

#[cfg(test)]
fn rendered(template: &str, page: &SeoPage) -> String {
    MetaTemplate::parse(template)
        .unwrap()
        .render(&demo_site(), page)
}

#[test]
fn every_variable_takes_its_value_from_the_site_or_the_page() {
    let mut page = post("Why internal links matter");
    page.author = Some(String::from("Asif"));
    assert_eq!(
        rendered(
            "{title} by {author} in {category} {sep} {site}: {tagline}",
            &page
        ),
        "Why internal links matter by Asif in On-page SEO | Field Notes on SEO: \
         Plain advice on search for small teams"
    );
    assert_eq!(rendered("{excerpt}", &page), page.excerpt);
}

#[test]
fn the_date_variables_use_the_words_of_the_pages_language() {
    let mut site = demo_site();
    site.today = Some(NaiveDate::from_ymd_opt(2026, 11, 2).unwrap());
    let mut words = TemplateWords::default();
    words.months[10] = String::from("Novembre");
    words.date_format = String::from("{day} {month} {year}");
    site.words.insert(Locale::try_new("fr").unwrap(), words);
    let mut page = post("Les liens");
    page.published = Some(Utc.with_ymd_and_hms(2026, 9, 14, 0, 0, 0).unwrap());
    page.modified = Some(Utc.with_ymd_and_hms(2026, 10, 1, 0, 0, 0).unwrap());
    let template = MetaTemplate::parse("{date}, {modified}, {currentmonth} {currentyear}").unwrap();
    assert_eq!(
        template.render(&site, &page),
        "14 September 2026, 1 October 2026, November 2026"
    );
    page.language = Locale::try_new("fr").unwrap();
    assert_eq!(
        template.render(&site, &page),
        "14 September 2026, 1 October 2026, Novembre 2026"
    );
}

#[test]
fn the_list_and_post_variables_take_their_values_from_the_page() {
    let mut page = post("Links");
    page.tags = vec![String::from("anchor text"), String::from("crawl")];
    page.focus_keyword = Some(String::from("internal links"));
    page.search_phrase = Some(String::from("sitemaps"));
    assert_eq!(
        rendered("{tag} {sep} {focuskw}", &page),
        "anchor text | internal links"
    );
    assert_eq!(
        rendered("Results for {searchphrase}", &page),
        "Results for sitemaps"
    );
    let mut tag = SeoPage::new(
        PageKind::Tag,
        Locale::try_new("en").unwrap(),
        UrlPath::try_new("/tag/crawl/").unwrap(),
        "Crawl",
    );
    tag.term_description = Some(String::from("How search engines find pages."));
    tag.page_number = PageNumber::new(2, 5);
    assert_eq!(
        rendered("{term} {page} {sep} {site}: {term_description}", &tag),
        "Crawl Page 2 of 5 | Field Notes on SEO: How search engines find pages."
    );
    tag.page_number = PageNumber::new(1, 5);
    assert_eq!(rendered("{term} {page}", &tag), "Crawl");
    assert_eq!(PageNumber::new(6, 5), None);
    assert_eq!(rendered("{term}", &page), "");
}

#[test]
fn the_separator_is_the_site_setting() {
    let mut site = demo_site();
    site.separator = String::from("·");
    let template = MetaTemplate::parse("{title} {sep} {site}").unwrap();
    assert_eq!(
        template.render(&site, &post("Anchor text")),
        "Anchor text · Field Notes on SEO"
    );
}

#[test]
fn a_missing_value_leaves_no_dangling_separator_or_double_space() {
    let mut page = post("");
    page.category = None;
    assert_eq!(
        rendered("{title} {sep} {category} {sep} {site}", &page),
        "Field Notes on SEO"
    );
    assert_eq!(rendered("  {title}   {sep}  ", &post("Links")), "Links");
}

#[test]
fn a_template_with_an_unknown_variable_is_refused() {
    let error = MetaTemplate::parse("{title} {sep} {sitename}").unwrap_err();
    assert!(
        matches!(&error, SeoError::TemplateVariable { name, .. } if name == "sitename"),
        "{error:?}"
    );
}

#[test]
fn a_template_with_a_lone_brace_is_refused() {
    for bad in [
        "{title {sep} {site}",
        "{title} } {site}",
        "{title",
        "{{title}}",
    ] {
        let error = MetaTemplate::parse(bad).unwrap_err();
        assert!(
            matches!(error, SeoError::TemplateBrace { .. }),
            "{bad:?}: {error:?}"
        );
    }
}

#[test]
fn a_template_keeps_its_source_as_written() {
    let source = "{title} {sep} {category} {sep} {site}";
    assert_eq!(MetaTemplate::parse(source).unwrap().as_str(), source);
    assert_eq!(
        MetaTemplate::parse("About us").unwrap().to_string(),
        "About us"
    );
}

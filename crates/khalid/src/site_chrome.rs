//! What every page of one language shows around its content: the site's
//! name, stylesheet, reader script, menus and announcement, from `site.toml`.

use khalid_core::{Config, FooterGroup, Locale, MenuLink, Mode};
use khalid_render::{AnnouncementBar, Nav, NavGroup, NavLink, RenderSite};

use crate::build_pages::path_of;
use crate::{assets, build_ai_page, build_reader};

/// The site as pages in `locale` show it, linking the stylesheet at
/// `stylesheet` and the reader script when a feature on uses it.
pub(crate) fn render_site(config: &Config, locale: &Locale, stylesheet: &str) -> RenderSite {
    let features = build_reader::features(config.reader());
    let nav = config.nav();
    RenderSite {
        name: config.name().to_owned(),
        locale: locale.clone(),
        home: path_of(config, locale, ""),
        stylesheet: stylesheet.to_owned(),
        nav: Nav {
            primary: menu(&nav.primary, locale),
            footer: footer(config, &nav.footer, locale),
            footer_groups: footer_groups(&nav.footer_groups, locale),
        },
        announcement: announcement(config, locale),
        main_site: None,
        share_image: None,
        script: assets::script_tag(config, &features),
        features,
        search: None,
        subscribe: subscribe(config),
    }
}

/// Where a template's newsletter form posts: the site's own sign-up
/// route, which `khalid serve` answers only on a server site with `[mail]`.
/// A static site, or one that sends no mail, takes no sign-ups.
fn subscribe(config: &Config) -> Option<String> {
    let takes_sign_ups = config.mode() == Mode::Server && config.mail().is_some();
    takes_sign_ups.then(|| format!("{}subscribe", config.mount()))
}

/// The links of `links` that pages in `locale` show, in the order written:
/// those for every language and those for `locale`.
fn menu(links: &[MenuLink], locale: &Locale) -> Vec<NavLink> {
    let shown = |link: &&MenuLink| link.language.as_ref().is_none_or(|only| only == locale);
    let link = |link: &MenuLink| NavLink {
        label: link.label.to_string(),
        href: link.href.to_string(),
    };
    links.iter().filter(shown).map(link).collect()
}

/// The footer links of `links` for `locale`, then the AI info page when it
/// is on (P3), so a reader or a model finds it from anywhere; the header
/// stays as the site's author set it. The page is built in the main
/// language alone, so every language links to that copy.
fn footer(config: &Config, links: &[MenuLink], locale: &Locale) -> Vec<NavLink> {
    let mut footer = menu(links, locale);
    if !config.ai_page().off {
        footer.push(NavLink {
            label: "AI info".to_owned(),
            href: path_of(config, config.default_language(), build_ai_page::REST),
        });
    }
    footer
}

/// The footer groups pages in `locale` show, in the order written: those
/// for every language and those for `locale`, each with its links for
/// `locale`. A group left with no link shows nothing.
fn footer_groups(groups: &[FooterGroup], locale: &Locale) -> Vec<NavGroup> {
    let shown = |group: &&FooterGroup| group.language.as_ref().is_none_or(|only| only == locale);
    let group = |group: &FooterGroup| NavGroup {
        heading: group.heading.to_string(),
        links: menu(&group.links, locale),
    };
    let groups = groups.iter().filter(shown).map(group);
    groups.filter(|group| !group.links.is_empty()).collect()
}

/// The announcement pages in `locale` show, if `site.toml` has one for
/// every language or for `locale`.
fn announcement(config: &Config, locale: &Locale) -> Option<AnnouncementBar> {
    let bar = config.announcement()?;
    let shown = bar.language.as_ref().is_none_or(|only| only == locale);
    shown.then(|| AnnouncementBar {
        text: bar.text.to_string(),
        href: bar.href.as_ref().map(ToString::to_string),
    })
}

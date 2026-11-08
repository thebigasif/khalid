//! The privacy page of each language at `<mount>privacy/` (privacy plan
//! U9, R20 and R21): written from the site's settings and the features it
//! has on, so it changes when they do. An owner page with the slug
//! `privacy` replaces it, and shows the same facts where it writes
//! `::privacy-facts`. The owner's words are never read as a template.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use khalid_content::{EMBED_PROVIDERS, EmbedProvider, PRIVACY_FACTS_MARK, SiteContent};
use khalid_core::{Config, Locale, Mode, UserMessage};
use khalid_render::{
    BodyHtml, CookiePurpose, Lasts, PageBody, PageKind, PrivacyCategory, PrivacyContact,
    PrivacyCookie, PrivacyFacts, PrivacyVendor, RenderPrivacy, Renderer,
};
use khalid_seo::PageKind as SeoKind;
use sha2::{Digest, Sha256};

use crate::build_pages::{self, LanguagePages, Page};
use crate::{build_lists, build_message, build_seo};

/// The end of the page's address, after the mount and the language.
const REST: &str = "privacy/";

/// The file in `data/` that keeps the day the facts last changed and their
/// hash, as `2026-11-08 <hash>`.
const KEPT: &str = "privacy-facts";

/// How many months a reader's choice lasts until `[privacy]` sets it.
const CONSENT_MONTHS: u8 = 6;

/// How many days the subscriber cookie lasts, as `khalid-newsletter` sets.
const READER_DAYS: u16 = 180;

/// How many days a post's unlock cookie lasts, as `khalid-api` sets.
const UNLOCK_DAYS: u16 = 30;

/// Adds the privacy page of each of `languages`, or places the facts in
/// the owner's own page at the same address. `site` is the site folder,
/// where `data/privacy-facts` keeps the day the facts last changed, and
/// `today` is the day a change is dated.
///
/// # Errors
///
/// Returns what, why and fix when the page's address cannot be made, or
/// when the theme cannot draw the facts.
pub(crate) fn add(
    (config, renderer): (&Config, &Renderer),
    content: &SiteContent,
    (site, today): (&Path, NaiveDate),
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    let mut facts = facts(config, content);
    facts.changed = changed_on(site, &facts, today);
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let path = build_pages::path_of(config, &locale, REST);
        match language
            .pages
            .iter_mut()
            .find(|page| page.meta.path == path)
        {
            Some(own) => place(renderer, &locale, &facts, own)?,
            None => language
                .pages
                .push(generated(config, renderer, &locale, &facts)?),
        }
    }
    Ok(())
}

/// The generated privacy page of `locale`.
fn generated(
    config: &Config,
    renderer: &Renderer,
    locale: &Locale,
    facts: &PrivacyFacts,
) -> Result<Page, UserMessage> {
    let title = build_lists::text(renderer, locale, "privacy_title")?;
    let description = build_lists::text(renderer, locale, "privacy_description")?
        .replace("{site}", config.name());
    let path = build_pages::path_of(config, locale, REST);
    let meta = build_pages::meta(
        config,
        title.clone(),
        description.clone(),
        path,
        PageKind::Website,
    )?;
    let mut seo = build_seo::page(SeoKind::Page, locale, REST, &title)?;
    seo.excerpt = description;
    seo.translation = build_lists::list_key("privacy")?;
    Ok(Page {
        file: format!("{}{REST}index.html", build_pages::prefix(config, locale)),
        meta,
        body: PageBody::Privacy(Box::new(RenderPrivacy {
            title,
            facts: facts.clone(),
        })),
        seo,
        source: None,
        template: None,
        head_code: None,
        lead: None,
    })
}

/// Swaps the `::privacy-facts` mark in the owner's page `own` for the facts
/// as the theme draws them in `locale`. A page without the mark keeps only
/// the owner's words.
fn place(
    renderer: &Renderer,
    locale: &Locale,
    facts: &PrivacyFacts,
    own: &mut Page,
) -> Result<(), UserMessage> {
    let PageBody::Page(page) = &mut own.body else {
        return Ok(());
    };
    if !page.body.as_str().contains(PRIVACY_FACTS_MARK) {
        return Ok(());
    }
    let drawn = renderer
        .privacy_facts(locale, facts)
        .map_err(|error| build_message::render(&error))?;
    let placed = page.body.as_str().replace(PRIVACY_FACTS_MARK, &drawn);
    page.body = BodyHtml::from_sanitised(placed);
    Ok(())
}

/// What the privacy page says about the site in `config` with `content`,
/// its day of change still empty.
pub(crate) fn facts(config: &Config, content: &SiteContent) -> PrivacyFacts {
    let server = config.mode() == Mode::Server;
    let bodies = || {
        (content.posts.iter().map(|post| post.body.as_str()))
            .chain(content.pages.iter().map(|page| page.body.as_str()))
    };
    let newsletter = server
        && (config.mail().is_some()
            || bodies().any(|body| body.contains("::newsletter") || body.contains("::gated")));
    let forms = server && bodies().any(|body| body.contains(":::form"));
    let passwords = server && (content.posts.iter()).any(|post| post.front.password_hash.is_some());
    let mut cookies = vec![PrivacyCookie {
        name: khalid_server::CONSENT_COOKIE.to_owned(),
        purpose: CookiePurpose::Consent,
        lasts: Lasts::Months(CONSENT_MONTHS),
    }];
    if newsletter {
        cookies.push(PrivacyCookie {
            name: "khalid_reader".to_owned(),
            purpose: CookiePurpose::Reader,
            lasts: Lasts::Days(READER_DAYS),
        });
    }
    if passwords {
        cookies.push(PrivacyCookie {
            name: "khalid_unlock_*".to_owned(),
            purpose: CookiePurpose::Unlock,
            lasts: Lasts::Days(UNLOCK_DAYS),
        });
    }
    let code = config.code();
    let vendors = if code.waits_for_consent() {
        vec![PrivacyVendor {
            name: None,
            category: PrivacyCategory::Marketing,
            policy: None,
        }]
    } else {
        Vec::new()
    };
    let visitor_log = config.visitor_log();
    PrivacyFacts {
        site: config.name().to_owned(),
        contact: contact(config),
        server,
        cookies,
        vendors,
        quiet: Vec::new(),
        embeds: embedded(bodies()),
        comments: server,
        newsletter,
        forms,
        visitor_log_days: (server && visitor_log.on).then_some(visitor_log.keep_days),
        consent_months: CONSENT_MONTHS,
        gpc_rejects_all: false,
        external_tool: None,
        changed: String::new(),
    }
}

/// Who readers write to about their data, from `[privacy]`. `None` until
/// the owner fills it in.
fn contact(_config: &Config) -> Option<PrivacyContact> {
    None
}

/// What `khalid doctor` and `khalid build` tell the owner when the privacy
/// page names no one to write to. The public page never says it.
pub(crate) fn missing_contact(config: &Config) -> Option<UserMessage> {
    contact(config).is_none().then(|| {
        UserMessage::new(
            "The privacy page has no contact details.",
            "Readers need a way to ask about their data, and privacy law asks for one.",
            "Add your name and email on the admin Privacy screen, under Contact.",
        )
    })
}

/// The names of the players the media blocks in `bodies` may load, in the
/// order of [`EMBED_PROVIDERS`].
fn embedded<'a>(bodies: impl Iterator<Item = &'a str>) -> Vec<String> {
    let media: Vec<&str> = bodies
        .flat_map(str::lines)
        .map(str::trim_start)
        .filter(|line| {
            ["::embed", "::video", "::audio"]
                .iter()
                .any(|b| line.starts_with(b))
        })
        .collect();
    EMBED_PROVIDERS
        .into_iter()
        .filter_map(known)
        .filter(|(_, hosts)| {
            media
                .iter()
                .any(|line| hosts.iter().any(|h| line.contains(h)))
        })
        .map(|(name, _)| name.to_owned())
        .collect()
}

/// The name a reader knows `provider` by, and the hosts its links use.
fn known(provider: EmbedProvider) -> Option<(&'static str, &'static [&'static str])> {
    match provider {
        EmbedProvider::YouTube => Some(("YouTube", &["youtube.com/", "youtu.be/"])),
        EmbedProvider::Vimeo => Some(("Vimeo", &["vimeo.com/"])),
        EmbedProvider::Spotify => Some(("Spotify", &["spotify.com/"])),
        EmbedProvider::SoundCloud => Some(("SoundCloud", &["soundcloud.com/"])),
        EmbedProvider::CodePen => Some(("CodePen", &["codepen.io/"])),
        EmbedProvider::Loom => Some(("Loom", &["loom.com/"])),
        _ => None,
    }
}

/// The day `facts` last changed: the day kept in `data/privacy-facts` while
/// their hash is the one kept beside it, or else `today`, which is kept
/// for next time. A failed write costs only the date, so it is logged.
fn changed_on(site: &Path, facts: &PrivacyFacts, today: NaiveDate) -> String {
    let hash = hash(facts);
    let file = kept_file(site);
    let kept = fs::read_to_string(&file).unwrap_or_default();
    if let Some((day, kept_hash)) = kept.trim().split_once(' ')
        && kept_hash == hash
        && NaiveDate::parse_from_str(day, "%Y-%m-%d").is_ok()
    {
        return day.to_owned();
    }
    let day = today.format("%Y-%m-%d").to_string();
    let written = file
        .parent()
        .map_or(Ok(()), fs::create_dir_all)
        .and_then(|()| fs::write(&file, format!("{day} {hash}\n")));
    if let Err(error) = written {
        tracing::warn!(path = %file.display(), %error, "could not keep when the privacy facts changed");
    }
    day
}

/// A short hash of everything `facts` says.
fn hash(facts: &PrivacyFacts) -> String {
    let said = serde_json::to_vec(facts).unwrap_or_default();
    let digest = Sha256::digest(&said);
    digest
        .get(..8)
        .unwrap_or_default()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// `data/privacy-facts` in the `site` folder.
fn kept_file(site: &Path) -> PathBuf {
    let mut path = site.to_path_buf();
    path.push("data");
    path.push(KEPT);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn some_facts() -> PrivacyFacts {
        let config = Config::from_toml(
            Path::new("site.toml"),
            "name = \"Notes\"\naddress = \"https://example.com\"\nmount = \"/\"\n\
             default_language = \"en\"\nlanguages = [\"en\"]\n",
            &khalid_core::Environment::empty(),
        )
        .unwrap();
        facts(&config, &SiteContent::default())
    }

    #[test]
    fn only_the_players_a_body_embeds_are_named() {
        let body = "Watch.\n\n::embed{src=\"https://www.youtube.com/watch?v=a\" title=\"A\"}\n";
        assert_eq!(embedded([body].into_iter()), ["YouTube"]);
        assert!(embedded(["See vimeo.com/1 for more."].into_iter()).is_empty());
    }

    #[test]
    fn the_day_moves_only_when_the_facts_change() {
        let site = tempfile::tempdir().unwrap();
        let mut facts = some_facts();
        assert_eq!(
            changed_on(site.path(), &facts, day("2026-11-01")),
            "2026-11-01"
        );
        assert_eq!(
            changed_on(site.path(), &facts, day("2026-11-08")),
            "2026-11-01"
        );
        facts.forms = true;
        assert_eq!(
            changed_on(site.path(), &facts, day("2026-11-09")),
            "2026-11-09"
        );
    }
}

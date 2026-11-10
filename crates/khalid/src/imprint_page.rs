//! The imprint of each language at `<mount>impressum/`, the Impressum
//! German law asks for (backlog row 36): written from `[privacy.contact]`
//! and `[privacy.imprint]`, only when the site has that table. An owner
//! page with the slug `impressum` replaces it.

use khalid_core::{Config, Locale, UserMessage};
use khalid_render::{PageBody, PageKind, RenderImprint, Renderer};
use khalid_seo::PageKind as SeoKind;

use crate::build_pages::{self, LanguagePages, Page};
use crate::{build_lists, build_seo, privacy_page};

/// The end of the page's address, after the mount and the language. The
/// German word, which readers in German-speaking countries look for.
pub(crate) const REST: &str = "impressum/";

/// Adds the imprint of each of `languages` that has no owner page at its
/// address, when `[privacy.imprint]` is set.
///
/// # Errors
///
/// Returns what, why and fix when the page's address cannot be made.
pub(crate) fn add(
    config: &Config,
    renderer: &Renderer,
    languages: &mut [LanguagePages],
) -> Result<(), UserMessage> {
    for language in languages.iter_mut() {
        let locale = language.site.locale.clone();
        let path = build_pages::path_of(config, &locale, REST);
        if language.pages.iter().any(|page| page.meta.path == path) {
            continue;
        }
        if let Some(page) = generated(config, renderer, &locale)? {
            language.pages.push(page);
        }
    }
    Ok(())
}

/// The generated imprint of `locale`, or `None` when the site has no
/// `[privacy.imprint]`. The config refuses one without a name, a postal
/// address and an email, so those are always there.
fn generated(
    config: &Config,
    renderer: &Renderer,
    locale: &Locale,
) -> Result<Option<Page>, UserMessage> {
    let privacy = config.privacy();
    let contact = &privacy.contact;
    let (Some(imprint), Some(name), Some(address), Some(email)) = (
        privacy.imprint.as_ref(),
        contact.name.clone(),
        contact.address.clone(),
        contact.email.clone(),
    ) else {
        return Ok(None);
    };
    let title = build_lists::text(renderer, locale, "imprint_title")?;
    let description = build_lists::text(renderer, locale, "imprint_description")?
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
    seo.translation = build_lists::list_key("impressum")?;
    Ok(Some(Page {
        file: format!("{}{REST}index.html", build_pages::prefix(config, locale)),
        meta,
        body: PageBody::Imprint(Box::new(RenderImprint {
            title,
            name,
            address,
            email,
            phone: imprint.phone.clone(),
            register: imprint.register.clone(),
            vat_id: imprint.vat_id.clone(),
            responsible: imprint.responsible.clone(),
            privacy: Some(build_pages::path_of(config, locale, privacy_page::REST)),
        })),
        seo,
        source: None,
        template: None,
        head_code: None,
        lead: None,
    }))
}

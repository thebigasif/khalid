//! `khalid build`: every page of the site rendered as HTML into an output
//! folder, `public/` in the site folder unless `--out` names another, with
//! the stylesheet every page links in `assets/`, the site's sitemaps,
//! `robots.txt`, `llms.txt` and feeds beside them, and each folder post's
//! files in its page's folder. A headless site gets those files and no
//! HTML.

use std::fs;
use std::path::{self, Path, PathBuf};

use chrono::NaiveDate;
use khalid_content::{ContentTree, SiteContent};
use khalid_core::{Clock, Config, SystemClock, UserMessage};
use khalid_lint::Report;
use khalid_render::{Asset, Renderer, Tokens};
use khalid_routes::Routes;
use serde::Serialize;

use crate::assets::PageStylesheets;
use crate::build_pages::LanguagePages;
use crate::cli::{Cli, Output};
use crate::exit::Exit;
use crate::newsletter_files::archive_pages;
use crate::page_code::PageCode;
use crate::post_files::{Assets, charts, library_photo};
use crate::post_photos::Shown;
use crate::seo_extras::{self, archives_off, term_seo};
use crate::site_page::HeadNonce;
use crate::static_mode::consent::{self as static_consent, Asking};
use crate::{
    assets, build_files, build_gate, build_lists, build_message, build_pages, build_redirects,
    build_seo, build_sitemap, categories, content_message, form_pages, look, own_descriptions,
    post_categories, post_files, post_images, post_lock, privacy_page, report, search_page,
    search_static, share_images, static_mode, trails,
};

#[path = "build_indexnow.rs"]
mod indexnow;
use indexnow::Told;

/// The output folder inside a site when `--out` is not given.
const PUBLIC: &str = "public";

/// The JSON shape of `khalid build --json`.
#[derive(Serialize)]
struct BuildBody {
    out: String,
    pages: usize,
    /// The folder of files for the root of the host, for a blog in a folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<String>,
    /// The features a static site has off, each with what, why and fix.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    off: Vec<static_mode::Off>,
    /// The posts the publish gate kept out, each with every finding.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    left_out: Vec<Report>,
    /// Things worth a look that are not problems, such as a privacy page
    /// with no contact details.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<UserMessage>,
    /// How many addresses were sent to `IndexNow`, when any request went out.
    #[serde(skip_serializing_if = "Option::is_none")]
    indexnow_sent: Option<usize>,
}

/// What one build wrote: how many pages, and the report and file of each
/// post the publish gate left out.
#[derive(Debug)]
pub(crate) struct Built {
    pub(crate) pages: usize,
    pub(crate) left_out: Vec<Report>,
    pub(crate) left_files: Vec<PathBuf>,
}

/// Renders the `site` folder, whose `config`, `routes` and `content` were
/// checked, into `out`, or `public/` in the site, then sends `IndexNow` what
/// changed unless `no_indexnow`.
pub(crate) fn run(
    site: &Path,
    (config, routes, content): (&Config, &Routes, &SiteContent),
    (out, no_indexnow): (Option<&Path>, bool),
    cli: &Cli,
) -> Exit {
    let output = cli.output;
    let out = out.map_or_else(|| within(site, PUBLIC), Path::to_path_buf);
    if inside(&out, &within(site, "content")) {
        return report::failure(output, &build_message::into_content(&out), Exit::Failure);
    }
    match into_folder(site, (config, routes, content), &out, &SystemClock, output) {
        Ok(built) => {
            let told = (!no_indexnow)
                .then(|| indexnow::tell((site, config), content, &built.left_files, cli))
                .flatten();
            success(output, &out, (built, told), config)
        }
        Err(exit) => exit,
    }
}

/// Builds the checked site into `out`, as `khalid build` does, without the
/// posts the publish gate refuses, and says how many pages it wrote and
/// which posts it left out. A failure is printed in `output` and comes
/// back as its exit.
pub(crate) fn into_folder(
    site: &Path,
    (config, routes, content): (&Config, &Routes, &SiteContent),
    out: &Path,
    clock: &dyn Clock,
    output: Output,
) -> Result<Built, Exit> {
    let fail = |message| report::failure(output, &message, Exit::Failure);
    let refused = build_gate::screen(site, (config, routes, content), clock).map_err(fail)?;
    let published = build_gate::published(content, &refused);
    let pages = build(site, config, routes, &published, out, clock).map_err(fail)?;
    let (left_files, left_out) = refused
        .into_iter()
        .map(|post| (post.path, post.report))
        .unzip();
    Ok(Built {
        pages,
        left_out,
        left_files,
    })
}

/// The renderer and stylesheet of the look `config` chose, with each
/// page's own stylesheet made from it, and the tokens the share images
/// take, for a site with pages. A headless site gets no renderer and the
/// default tokens, and its theme is never read.
fn theme(site: &Path, config: &Config) -> Result<(Option<Theme>, Tokens), UserMessage> {
    if !config.mode().has_pages() {
        return Ok((None, Tokens::default()));
    }
    let look = look::of(site, config)?;
    let tokens = look::tokens(site, &look)?;
    let renderer = Renderer::with_pack(site, &look::render_pack(&look, config))
        .map_err(|error| build_message::render(&error))?
        .with_brand(&tokens.brand);
    let css = assets::site_stylesheet(site, &look)?;
    let whole = String::from_utf8_lossy(css.body());
    let look_rules = (&tokens, look.pack().stylesheet());
    let sheets = PageStylesheets::new(config, &assets::href(config, &css), look_rules, &whole);
    Ok((Some((renderer, css, sheets)), tokens))
}

/// The pages of every language. With a theme, they link its stylesheet
/// `css` and preload the body font its `tokens` name; a headless site has
/// neither.
fn language_pages(
    site: &Path,
    (config, routes): (&Config, &Routes),
    content: &SiteContent,
    theme: Option<(&Asset, &Tokens)>,
    today: NaiveDate,
) -> Result<Vec<LanguagePages>, UserMessage> {
    let href = theme
        .map(|(css, _)| assets::href(config, css))
        .unwrap_or_default();
    let mut languages = build_pages::pages(config, routes, (content, &[]), &href, today)?;
    let tokens = theme.map(|(_, tokens)| tokens);
    assets::preload_body_font(site, config, tokens, &mut languages);
    Ok(languages)
}

/// The renderer, the whole stylesheet and each page's own stylesheet.
type Theme = (Renderer, Asset, PageStylesheets);

/// Renders every page into `out`, then writes the stylesheet they link and
/// the site's files beside them, and says how many pages were written. A
/// headless site gets the files only, and its theme is never read (ADR
/// "headless is a mode").
fn build(
    site: &Path,
    config: &Config,
    routes: &Routes,
    content: &SiteContent,
    out: &Path,
    clock: &dyn Clock,
) -> Result<usize, UserMessage> {
    let content = &*charts::read(site, content);
    let (theme, tokens) = theme(site, config)?;
    let today = clock.now().date_naive();
    let styled = theme.as_ref().map(|(_, css, _)| (css, &tokens));
    let mut languages = language_pages(site, (config, routes), content, styled, today)?;
    let renderer = theme.as_ref().map(|(renderer, ..)| renderer);
    static_mode::comments(config, &mut languages);
    form_pages::media_under(config, &mut languages);
    post_lock::lock(config, content, renderer, &mut languages);
    let categories = categories::of(site)?;
    let asking = Asking::of(config, &languages);
    if let Some((renderer, ..)) = &theme {
        search_page::add(config, renderer, &mut languages)?;
        archive_pages::add(site, config, (renderer, today), &mut languages)?;
        let sources = (content, categories.as_slice());
        build_lists::add((config, routes), renderer, sources, &mut languages)?;
        let found = (renderer, site, today);
        build_pages::discovery_pages((config, routes), found, content, &mut languages)?;
        build_sitemap::add(config, renderer, &mut languages)?;
        own_descriptions::fill(config, renderer, &mut languages)?;
    }
    let (gone, bare) = seo_extras::arrange(site, (config, routes), &mut languages)?;
    term_seo::note(&bare);
    // A headless site's heads carry the same trail as its pages would.
    trails::fill(site, (config, routes), &mut languages)?;
    post_categories::fill((config, routes), &categories, &mut languages);
    // The checked content keeps no assets, so the tree is read again for
    // them. A headless site gets them too: its feeds point at them.
    let tree = ContentTree::walk(site).map_err(|error| content_message::of(site, &error))?;
    let mut beside = post_files::attach(&Assets::of(&tree), &mut languages);
    beside.extend(library_photo::files(site, config, &languages));
    for file in &beside {
        copy(&post_files::on_disk(site, file)?, out, &file.file)?;
    }
    let mut shown = Shown::default();
    for size in post_images::attach(site, config, &beside, &mut languages, &mut shown)? {
        copy(&size.path, out, &size.file)?;
    }
    let seo_site = build_seo::site_on(config, today)?;
    let cards = share_images::attach(site, config, &tokens, &seo_site, &mut languages)?;
    for file in share_images::draw_now(site, cards)? {
        let mut from = site.to_path_buf();
        from.push("data");
        from.push(&file);
        copy(&from, out, &file)?;
    }
    let seo_pages = build_seo::heads(&seo_site, &mut languages)?;
    build_sitemap::fill(config, &seo_site, &seo_pages, &mut languages)?;
    let mut written = 0;
    let locks = static_mode::locks(config.mode())?;
    let mut code = PageCode::new(config, HeadNonce::Static);
    if let Some((renderer, css, sheets)) = &theme {
        write_asset(out, css)?;
        write_asset(out, &assets::reader_with_copies()?)?;
        if let Some((script, tag)) = static_consent::script(config, asking)? {
            code = code.with_consent_script(tag);
            write_asset(out, &script)?;
        }
        write_fonts(site, &tokens, out)?;
        written = write_html(out, (renderer, sheets), &languages, (locks.as_ref(), &code))?;
    }
    let redirects = build_redirects::file(site, config, routes, content)?;
    let redirects = archives_off::with_redirects(redirects, &gone);
    let files = build_files::files(
        &seo_site,
        &seo_pages,
        config.seo().indexnow.as_ref(),
        clock.now(),
    )?;
    let search = search_static::files(config, &languages)?;
    for file in files.iter().chain(&redirects).chain(&search) {
        write(out, &file.file, &file.text)?;
    }
    for file in build_files::root_files(&seo_site)? {
        let relative = format!("{}/{}", build_files::ROOT_FOLDER, file.file);
        write(out, &relative, &file.text)?;
    }
    let (gpc, honoured) = static_consent::gpc_file(config);
    write(out, &gpc, honoured)?;
    Ok(written)
}

/// Copies the file at `from` to `relative` inside `out`, making its folders
/// first.
fn copy(from: &Path, out: &Path, relative: &str) -> Result<(), UserMessage> {
    let target = within(out, relative);
    let made = target.parent().map_or(Ok(()), fs::create_dir_all);
    made.and_then(|()| fs::copy(from, &target).map(drop))
        .map_err(|error| build_message::write(&target, &error))
}

/// Writes `text` to `relative` inside `out`, making its folders first.
fn write(out: &Path, relative: &str, text: &str) -> Result<(), UserMessage> {
    let file = within(out, relative);
    let made = file.parent().map_or(Ok(()), fs::create_dir_all);
    made.and_then(|()| fs::write(&file, text))
        .map_err(|error| build_message::write(&file, &error))
}

/// Writes the site's self-hosted font files, the `*.woff2` files in
/// `theme/fonts/` that the tokens list, into `out/assets/fonts/`, where
/// the stylesheet's `@font-face` rules point (G20). Anything else in the
/// folder stays private.
fn write_fonts(site: &Path, tokens: &Tokens, out: &Path) -> Result<(), UserMessage> {
    let fonts = assets::fonts(site, tokens);
    if fonts.is_empty() {
        return Ok(());
    }
    let into = within(out, &format!("{}fonts/", assets::FOLDER));
    fs::create_dir_all(&into).map_err(|error| build_message::write(&into, &error))?;
    for font in fonts {
        let file = within(&into, &font.name);
        fs::write(&file, font.bytes).map_err(|error| build_message::write(&file, &error))?;
    }
    Ok(())
}

/// Writes `asset` into `out/assets/` with its Brotli and gzip copies beside
/// it, `.br` and `.gz`, so a web server can send them as they are.
fn write_asset(out: &Path, asset: &Asset) -> Result<(), UserMessage> {
    let folder = within(out, assets::FOLDER);
    let name = asset.name();
    let copies = [
        (name.to_owned(), asset.body()),
        (format!("{name}.br"), asset.brotli()),
        (format!("{name}.gz"), asset.gzip()),
    ];
    fs::create_dir_all(&folder).map_err(|error| build_message::write(&folder, &error))?;
    for (file, bytes) in copies {
        let file = within(&folder, &file);
        fs::write(&file, bytes).map_err(|error| build_message::write(&file, &error))?;
    }
    Ok(())
}

/// Writes each page of `languages` into `out`, linking its own stylesheet
/// from `sheets`, then the stylesheets the pages link, and says how many
/// pages were written.
fn write_html(
    out: &Path,
    (renderer, sheets): (&Renderer, &PageStylesheets),
    languages: &[LanguagePages],
    (locks, code): (Option<&khalid_static::Locks>, &PageCode),
) -> Result<usize, UserMessage> {
    let mut written = 0;
    for language in languages {
        for page in &language.pages {
            let html = page.rendered(renderer, &language.site, code)?;
            let html = static_mode::lock(locks, html, &language.site.locale, &page.file)?;
            write(out, &page.file, &sheets.fit(html).0)?;
            written += 1;
        }
    }
    for sheet in sheets.made() {
        let body = sheet.text().as_bytes().to_vec();
        let built = Asset::build("site", "css", "text/css; charset=utf-8", body)
            .map_err(|error| build_message::render(&error))?;
        write_asset(out, &built)?;
    }
    Ok(written)
}

/// `relative` inside `folder`. Every `relative` here is a fixed name, or a
/// path made from a checked slug and language, or an asset's hashed name.
fn within(folder: &Path, relative: &str) -> PathBuf {
    let mut path = folder.to_path_buf();
    path.push(relative);
    path
}

/// Whether `path` is `folder` or inside it, compared as absolute paths.
fn inside(path: &Path, folder: &Path) -> bool {
    match (path::absolute(path), path::absolute(folder)) {
        (Ok(path), Ok(folder)) => path.starts_with(folder),
        _ => false,
    }
}

/// Says which posts were left out and why, how many pages were written and
/// where, or for a headless site that only its files were, where the files
/// for the host's root are, for a static site which features are off, and
/// what `IndexNow` was `told`.
fn success(
    output: Output,
    out: &Path,
    (built, told): (Built, Option<Told>),
    config: &Config,
) -> Exit {
    let Built {
        pages, left_out, ..
    } = built;
    let shown = out.display().to_string();
    let mode = config.mode();
    let off = static_mode::off(mode);
    let root = (config.mount() != "/")
        .then(|| within(out, build_files::ROOT_FOLDER).display().to_string());
    let built = if mode.has_pages() {
        format!(
            "Built {pages} pages in {shown}.\n{}",
            static_mode::human(&off)
        )
    } else {
        format!(
            "Built the feeds, sitemaps and redirects in {shown}. The site is headless, so Khalid writes no pages.\n"
        )
    };
    let mut said = format!("{}{built}", build_gate::human(&left_out));
    if let Some(root) = &root {
        let host = config.address().as_str().trim_end_matches('/');
        said = format!(
            "{said}Search engines read robots.txt only at {host}/robots.txt, outside the blog's folder, so its copy is in {root}. Copy the files in that folder to the root of the main site.\n"
        );
    }
    let warnings: Vec<UserMessage> = (mode.has_pages())
        .then(|| privacy_page::missing_contact(config))
        .flatten()
        .into_iter()
        .collect();
    for warning in &warnings {
        let (what, why, fix) = (warning.what(), warning.why(), warning.fix());
        said = format!("{said}Worth a look: {what}\n  {why}\n  {fix}\n");
    }
    let indexnow_sent = told.as_ref().and_then(|told| told.sent);
    if let Some(told) = told {
        said = format!("{said}{}\n", told.line);
    }
    match output {
        Output::Human => report::text(&said, Exit::Success),
        Output::Json => report::json(
            &BuildBody {
                out: shown,
                pages,
                root,
                off,
                left_out,
                warnings,
                indexnow_sent,
            },
            Exit::Success,
        ),
    }
}

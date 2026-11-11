//! Pages and glossary terms kept as files in a small site folder.

use khalid_api::resources::{AdminPage, AdminTerm, NewPage, NewTerm, PageEdit, TermEdit};
use khalid_core::Environment;
use serde_json::json;

use super::*;

const SITE_TOML: &str = "name = \"Notes\"\naddress = \"https://example.com\"\n\
    default_language = \"en\"\nlanguages = [\"en\", \"de\"]\n";

const ABOUT: &str = "---\ntitle: About\nslug: about\ntranslation: about\n---\n\nWho writes here.\n";

fn within(dir: &Path, parts: &[&str]) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.extend(parts);
    path
}

/// A site in English and German with one English page, About.
fn site() -> (tempfile::TempDir, SiteEntries<AdminPage>) {
    let temp = tempfile::tempdir().unwrap();
    let toml = within(temp.path(), &["site.toml"]);
    let config = Config::from_toml(&toml, SITE_TOML, &Environment::empty()).unwrap();
    let pages = within(temp.path(), &["content", "en", "pages"]);
    fs::create_dir_all(&pages).unwrap();
    fs::write(within(&pages, &["about.md"]), ABOUT).unwrap();
    fs::write(within(&pages, &["_index.md"]), "---\ntitle: Pages\n---\n").unwrap();
    let pages = SiteEntries::new(temp.path(), &config);
    (temp, pages)
}

fn new_page(value: serde_json::Value) -> NewPage {
    serde_json::from_value(value).unwrap()
}

fn edit(value: serde_json::Value) -> PageEdit {
    serde_json::from_value(value).unwrap()
}

#[test]
fn pages_are_read_from_their_files_and_the_index_is_left_out() {
    let (site, pages) = site();
    let german = within(site.path(), &["content", "de", "pages"]);
    fs::create_dir_all(&german).unwrap();
    let ueber_uns = "---\ntitle: Über uns\nslug: ueber-uns\n---\nWir.\n";
    fs::write(within(&german, &["ueber-uns.md"]), ueber_uns).unwrap();
    let listed = pages.list_now().unwrap();
    let ids: Vec<_> = listed.iter().map(Entry::id).collect();
    assert_eq!(
        ids,
        ["en.about", "de.ueber-uns"],
        "the site's languages in order"
    );
    let json = serde_json::to_value(listed.first().unwrap()).unwrap();
    assert_eq!(json["title"], "About");
    assert_eq!(json["markdown"], "Who writes here.");
    assert!(pages.find("de.about").unwrap().is_none());
    assert!(pages.find("fr.about").unwrap().is_none());
}

#[test]
fn a_new_page_is_a_file_in_its_language_made_once() {
    let (site, pages) = site();
    let new = json!({"language": "de", "slug": "kontakt", "title": "Kontakt", "description": "Schreib uns.", "markdown": "Hallo."});
    let made = pages.create_now(&new_page(new.clone())).unwrap();
    assert_eq!(made.id(), "de.kontakt");
    let file = fs::read_to_string(within(
        site.path(),
        &["content", "de", "pages", "kontakt.md"],
    ));
    assert_eq!(
        file.unwrap(),
        "---\ntitle: \"Kontakt\"\nslug: kontakt\ndescription: \"Schreib uns.\"\n---\n\nHallo.\n"
    );
    let again = pages.create_now(&new_page(new)).unwrap_err();
    assert!(matches!(again, EntryError::Taken(id) if id == "de.kontakt"));
    let french = json!({"language": "fr", "slug": "contact", "title": "Contact", "markdown": "M."});
    let refused = pages.create_now(&new_page(french)).unwrap_err();
    assert!(matches!(
        refused,
        EntryError::Unsupported {
            field: "language",
            ..
        }
    ));
}

#[test]
fn an_edit_keeps_every_line_it_does_not_change() {
    let (site, pages) = site();
    let seen = pages.find("en.about").unwrap().unwrap().entry.updated_at();
    let change = json!({"title": "About us", "description": "Who we are.", "updated_at": seen});
    let saved = pages.update_now("en.about", &edit(change)).unwrap();
    assert_eq!(serde_json::to_value(&saved).unwrap()["title"], "About us");
    let file = fs::read_to_string(within(site.path(), &["content", "en", "pages", "about.md"]));
    assert_eq!(
        file.unwrap(),
        "---\ntitle: \"About us\"\nslug: about\ntranslation: about\ndescription: \"Who we are.\"\n---\n\nWho writes here.\n"
    );
    let cleared = json!({"description": "", "updated_at": saved.updated_at()});
    let saved = pages.update_now("en.about", &edit(cleared)).unwrap();
    assert_eq!(
        serde_json::to_value(&saved).unwrap()["description"],
        json!(null)
    );
}

#[test]
fn a_copy_older_than_the_file_is_stale_even_after_a_hand_edit() {
    let (site, pages) = site();
    let seen = pages.find("en.about").unwrap().unwrap().entry.updated_at();
    let path = within(site.path(), &["content", "en", "pages", "about.md"]);
    let changed = fs::metadata(&path).unwrap().modified().unwrap();
    let file = fs::File::options().write(true).open(&path).unwrap();
    let later = changed + std::time::Duration::from_secs(5);
    file.set_modified(later).unwrap();
    let change = json!({"title": "Lost", "updated_at": seen});
    let refused = pages.update_now("en.about", &edit(change)).unwrap_err();
    assert!(matches!(refused, EntryError::Stale { .. }));
    let refused = pages.delete_now("en.about", seen).unwrap_err();
    assert!(matches!(refused, EntryError::Stale { .. }));
    assert_eq!(fs::read_to_string(&path).unwrap(), ABOUT);
}

#[test]
fn a_glossary_term_is_written_in_the_shape_the_site_reads() {
    let (site, _) = site();
    let toml = within(site.path(), &["site.toml"]);
    let config = Config::from_toml(&toml, SITE_TOML, &Environment::empty()).unwrap();
    let glossary = SiteEntries::<AdminTerm>::new(site.path(), &config);
    let new = json!({"slug": "anchor-text", "term": "Anchor text", "translation": "anchor", "markdown": "The words a link sits on."});
    let made = glossary
        .create_now(&serde_json::from_value::<NewTerm>(new).unwrap())
        .unwrap();
    let path = within(
        site.path(),
        &["content", "en", "glossary", "anchor-text.md"],
    );
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "---\nterm: \"Anchor text\"\nslug: anchor-text\ntranslation: \"anchor\"\n---\n\nThe words a link sits on.\n"
    );
    let change = json!({"term": "Anchor", "translation": "", "updated_at": made.updated_at()});
    let change: TermEdit = serde_json::from_value(change).unwrap();
    glossary.update_now("en.anchor-text", &change).unwrap();
    let file = fs::read_to_string(&path).unwrap();
    assert_eq!(
        file,
        "---\nterm: \"Anchor\"\nslug: anchor-text\n---\n\nThe words a link sits on.\n"
    );
    let text = PostText::parse(&file).unwrap();
    let front: khalid_content::GlossaryFront = crate::entry_files::front(&text).unwrap();
    assert_eq!(front.term.as_deref(), Some("Anchor"), "the site reads it");
}

#[test]
fn a_delete_on_the_latest_copy_removes_the_file_or_its_folder() {
    let (site, pages) = site();
    let seen = pages.find("en.about").unwrap().unwrap().entry.updated_at();
    pages.delete_now("en.about", seen).unwrap();
    assert!(!within(site.path(), &["content", "en", "pages", "about.md"]).exists());
    let folder = within(site.path(), &["content", "en", "pages", "team"]);
    fs::create_dir_all(&folder).unwrap();
    fs::write(
        within(&folder, &["index.md"]),
        "---\ntitle: Team\n---\nUs.\n",
    )
    .unwrap();
    fs::write(within(&folder, &["photo.jpg"]), "jpeg").unwrap();
    let team = pages.find("en.team").unwrap().unwrap().entry;
    pages.delete_now("en.team", team.updated_at()).unwrap();
    assert!(!folder.exists());
    assert!(matches!(
        pages.delete_now("en.team", team.updated_at()),
        Err(EntryError::NotFound)
    ));
}

#[tokio::test]
async fn a_deleted_page_folder_waits_in_the_trash_until_its_address_is_free() {
    use khalid_api::post_trash::{PostTrash, Restored, TrashError, TrashedKind};

    let (site, pages) = site();
    let toml = within(site.path(), &["site.toml"]);
    let config = Config::from_toml(&toml, SITE_TOML, &Environment::empty()).unwrap();
    let at = chrono::TimeZone::with_ymd_and_hms(&Utc, 2026, 11, 11, 9, 0, 0).unwrap();
    let clock = khalid_core::FixedClock::new(at);
    let posts = SitePosts::new(site.path(), config, clock);
    let pages = pages.with_trash(posts.clone());
    let folder = within(site.path(), &["content", "en", "pages", "team"]);
    fs::create_dir_all(&folder).unwrap();
    fs::write(
        within(&folder, &["index.md"]),
        "---\ntitle: Team\n---\nUs.\n",
    )
    .unwrap();
    fs::write(within(&folder, &["photo.jpg"]), "jpeg").unwrap();
    let team = pages.find("en.team").unwrap().unwrap().entry;
    pages.delete_now("en.team", team.updated_at()).unwrap();
    assert!(!folder.exists());

    let trashed = posts.trashed().await.unwrap();
    let copy = trashed.first().unwrap();
    assert_eq!((copy.id.as_str(), copy.title.as_str()), ("en.team", "Team"));
    assert_eq!(copy.kind, TrashedKind::Page);
    let kept = [
        "trash",
        &copy.key,
        "content",
        "en",
        "pages",
        "team",
        "photo.jpg",
    ];
    assert!(within(site.path(), &kept).is_file());

    // A new page took the address, under another file name.
    let other = within(site.path(), &["content", "en", "pages", "us.md"]);
    fs::write(&other, "---\ntitle: Us\nslug: team\n---\nWe.\n").unwrap();
    let taken = posts.restore(&copy.key).await;
    assert!(matches!(taken, Err(TrashError::Taken { id }) if id == "en.team"));
    fs::remove_file(&other).unwrap();
    assert_eq!(posts.restore(&copy.key).await.unwrap(), Restored::Page);
    assert!(within(&folder, &["photo.jpg"]).is_file());
    assert_eq!(posts.trashed().await.unwrap(), []);
}

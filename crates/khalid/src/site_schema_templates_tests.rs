//! Schema templates written into `site.toml` read back the same, and every
//! other line of the file stays as it was.

use std::fs;
use std::path::PathBuf;

use chrono::TimeDelta;
use khalid_api::resources::NonBlank;
use khalid_api::schema_template_store::{
    AdminSchemaTemplate, TemplateStoreError, TemplateTarget, TemplatesChange,
};
use khalid_core::{Config, Environment};

use super::SiteSchemaTemplates;

/// A copy of the demo site's `site.toml` with a comment of the owner's,
/// in a folder with a `guides` collection, and its templates as a server
/// started on it sees them.
fn site() -> (tempfile::TempDir, SiteSchemaTemplates) {
    site_with("", &Environment::empty())
}

/// [`site`], with `more` at the end of `site.toml`, for a server started
/// with `env`.
fn site_with(more: &str, env: &Environment) -> (tempfile::TempDir, SiteSchemaTemplates) {
    let folder = tempfile::tempdir().unwrap();
    let mut demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    demo.push("../../fixtures/demo-site/site.toml");
    let text = fs::read_to_string(demo).unwrap();
    let text = format!("{text}\n# Kept by hand.\n{more}");
    let mut file = folder.path().to_path_buf();
    file.push("site.toml");
    fs::write(&file, &text).unwrap();
    let mut guides = folder.path().to_path_buf();
    guides.push("content/en/guides");
    fs::create_dir_all(&guides).unwrap();
    guides.push("a-guide.md");
    fs::write(&guides, "---\ntitle: A guide\n---\n\nText.\n").unwrap();
    let running = Config::from_toml(&file, &text, env).unwrap();
    let templates = SiteSchemaTemplates::new(folder.path(), &running, env);
    (folder, templates)
}

/// A `Course` for every guide, made by the site.
fn course() -> AdminSchemaTemplate {
    let guides = NonBlank::try_new("guides").unwrap();
    let mut course = AdminSchemaTemplate::new(TemplateTarget::Collection, guides, "Course");
    course
        .properties
        .insert("provider".to_owned(), "{site}".to_owned());
    course
}

#[test]
fn templates_written_into_site_toml_read_back_and_keep_the_rest() {
    let (folder, templates) = site();
    let before = templates.read_now().unwrap();
    assert_eq!(before.templates, Vec::new());
    assert!(!before.restart_needed);
    assert_eq!(before.collections, ["guides", "posts"]);

    let change = TemplatesChange {
        templates: vec![course()],
        updated_at: before.updated_at,
    };
    let after = templates.replace_now(&change).unwrap();
    assert_eq!(after.templates, vec![course()]);
    assert!(!after.restart_needed, "the pages use templates once saved");
    let mut file = folder.path().to_path_buf();
    file.push("site.toml");
    let text = fs::read_to_string(file).unwrap();
    assert!(text.contains("# Kept by hand.\n"), "{text}");
    let written = "[[seo.schema]]\ncollection = \"guides\"\ntype = \"Course\"\n\n\
                   [seo.schema.properties]\nprovider = \"{site}\"\n";
    assert!(text.contains(written), "{text}");

    let none = TemplatesChange {
        templates: Vec::new(),
        updated_at: after.updated_at,
    };
    let cleared = templates.replace_now(&none).unwrap();
    assert_eq!(cleared.templates, Vec::new());
    assert!(!cleared.restart_needed);
}

#[test]
fn a_change_made_on_an_old_copy_writes_nothing() {
    let (_folder, templates) = site();
    let before = templates.read_now().unwrap();
    let change = TemplatesChange {
        templates: vec![course()],
        updated_at: before.updated_at - TimeDelta::seconds(1),
    };
    let refused = templates.replace_now(&change).unwrap_err();
    assert!(
        matches!(refused, TemplateStoreError::Stale { .. }),
        "{refused}"
    );
    assert_eq!(templates.read_now().unwrap().templates, Vec::new());
}

#[test]
fn a_site_that_sends_mail_through_resend_saves_templates_with_the_start_key() {
    let mail = "\n[mail]\nfrom = \"news@example.com\"\n\n[mail.resend]\n";
    let env = Environment::from_pairs(&[(Environment::RESEND_API_KEY, "made-for-this-test")]);
    let (_folder, templates) = site_with(mail, &env);
    let before = templates.read_now().unwrap();
    let change = TemplatesChange {
        templates: vec![course()],
        updated_at: before.updated_at,
    };
    let after = templates.replace_now(&change).unwrap();
    assert_eq!(after.templates, vec![course()]);
    assert!(!after.restart_needed);
}

#[test]
fn a_start_only_setting_changed_by_hand_still_waits_for_a_restart() {
    let (folder, templates) = site();
    let mut file = folder.path().to_path_buf();
    file.push("site.toml");
    let text = fs::read_to_string(&file).unwrap();
    fs::write(&file, text.replace("www.example.com", "www.example.net")).unwrap();
    assert!(templates.read_now().unwrap().restart_needed);
}

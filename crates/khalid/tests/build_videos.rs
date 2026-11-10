//! `khalid build` writes a video sitemap of every post's video embeds and
//! lists it in the sitemap index, under the mount (KH-SEO-5, G16).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

/// `rel` under `base`.
#[cfg(test)]
fn at(base: &Path, rel: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    path.push(rel);
    path
}

/// Copies the folder `from` into `to`.
#[cfg(test)]
fn copy(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap().map(Result::unwrap) {
        let target = at(to, &entry.file_name().to_string_lossy());
        if entry.path().is_dir() {
            copy(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// A copy of the demo site, which is mounted at `/blog/`.
#[cfg(test)]
fn demo_copy() -> TempDir {
    let site = tempfile::tempdir().unwrap();
    let demo = at(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "../../fixtures/demo-site",
    );
    copy(&demo, site.path());
    site
}

/// The demo site with a `YouTube` and a Vimeo embed at the end of "Fixing
/// slow pages", whose lead image stands in for Vimeo's still.
#[cfg(test)]
fn site_with_embeds() -> TempDir {
    let site = demo_copy();
    let post = at(
        site.path(),
        "content/en/posts/2026-10-05-fixing-slow-pages/index.md",
    );
    let mut text = fs::read_to_string(&post).unwrap();
    text.push_str(
        "\n::embed{src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" title=\"How search works\"}\n\
         \n::embed{src=\"https://vimeo.com/76979871\" title=\"A short film\"}\n",
    );
    fs::write(post, text).unwrap();
    site
}

/// `site` built into a new folder.
#[cfg(test)]
fn build(site: &Path) -> TempDir {
    let out = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_khalid"))
        .args(["--site", site.to_str().unwrap(), "build", "--out"])
        .arg(out.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    out
}

#[cfg(test)]
fn read(out: &TempDir, rel: &str) -> String {
    fs::read_to_string(at(out.path(), rel)).unwrap_or_else(|error| panic!("{rel}: {error}"))
}

#[test]
fn the_video_sitemap_lists_each_embed_by_its_player() {
    let site = site_with_embeds();
    let out = build(site.path());
    let videos = read(&out, "sitemaps/videos-en-1.xml");
    for text in [
        "<loc>https://www.example.com/blog/fixing-slow-pages/</loc>",
        "<video:thumbnail_loc>https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg</video:thumbnail_loc>",
        "<video:title>How search works</video:title>",
        "<video:player_loc>https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ</video:player_loc>",
        "<video:title>A short film</video:title>",
        "<video:thumbnail_loc>https://www.example.com/blog/fixing-slow-pages/page-weight.png</video:thumbnail_loc>",
        "<video:player_loc>https://player.vimeo.com/video/76979871</video:player_loc>",
        "<video:description>Slow pages lose readers before they read a word.",
    ] {
        assert!(videos.contains(text), "{text} in:\n{videos}");
    }
    assert_eq!(videos.matches("<video:description>").count(), 2, "{videos}");
    let index = read(&out, "sitemap.xml");
    assert!(
        index.contains("<loc>https://www.example.com/blog/sitemaps/videos-en-1.xml</loc>"),
        "{index}"
    );
    let posts = read(&out, "sitemaps/posts-en-1.xml");
    assert!(!posts.contains("<video:video>"), "listed once: {posts}");
}

#[test]
fn a_site_without_videos_writes_no_video_sitemap() {
    let site = demo_copy();
    let out = build(site.path());
    assert!(!at(out.path(), "sitemaps/videos-en-1.xml").exists());
    assert!(!read(&out, "sitemap.xml").contains("videos-"));
}

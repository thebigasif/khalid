//! The HTML of the media blocks: video, audio, file download, gallery and
//! embed.

use khalid_core::{AbsoluteUrl, Locale};

use super::{Image, Link, Renderer, attribute};
use crate::external_media::{MEDIA_CARD_END, MEDIA_CARD_START, MEDIA_END, MEDIA_FORM, MEDIA_START};
use crate::{GalleryImage, StructuredBlock, VideoBlock, embed};

/// The height of an allowed site's frame when its embed does not say, in
/// pixels: room for a calculator or a chart with its controls.
const DEFAULT_HEIGHT: u16 = 480;

impl Renderer<'_> {
    /// Writes a video with controls that loads nothing until played, its
    /// file's link for browsers that cannot play it, and its caption.
    pub(super) fn video(&mut self, video: &VideoBlock) {
        let src = attribute(&video.src);
        let title = attribute(&video.title);
        let caption = attribute(video.description.as_deref().unwrap_or(&video.title));
        self.push(&[
            "<figure class=\"video\">\n<video controls preload=\"none\" poster=\"",
            &attribute(&video.poster),
            "\" aria-label=\"",
            &title,
            "\"><source src=\"",
            &src,
            "\" /><a href=\"",
            &src,
            "\">",
            &title,
            "</a></video>\n<figcaption>",
            &caption,
            "</figcaption>\n</figure>\n",
        ]);
        self.facts
            .structured
            .push(StructuredBlock::Video(video.clone()));
    }

    /// Writes a sound file with controls under its title.
    pub(super) fn audio(&mut self, src: &str, title: &str) {
        let src = attribute(src);
        let title = attribute(title);
        self.push(&[
            "<figure class=\"audio\">\n<figcaption>",
            &title,
            "</figcaption>\n<audio controls preload=\"none\" src=\"",
            &src,
            "\"><a href=\"",
            &src,
            "\">",
            &title,
            "</a></audio>\n</figure>\n",
        ]);
    }

    /// Writes a link that downloads a file.
    pub(super) fn file(&mut self, src: &str, title: &str) {
        self.push(&[
            "<p class=\"file\"><a href=\"",
            &attribute(src),
            "\" download>",
            &attribute(title),
            "</a></p>\n",
        ]);
        self.facts.links.push(Link {
            href: src.to_owned(),
            text: title.to_owned(),
            internal: !src.contains(':'),
        });
    }

    /// Writes a grid of pictures, each loaded as it scrolls into view.
    pub(super) fn gallery(&mut self, images: &[GalleryImage]) {
        self.html.push_str("<div class=\"gallery\">\n");
        for image in images {
            self.push(&[
                "<figure><img src=\"",
                &attribute(&image.src),
                "\" alt=\"",
                &attribute(&image.alt),
                "\" loading=\"lazy\" decoding=\"async\" /></figure>\n",
            ]);
            self.facts.images.push(Image {
                src: image.src.clone(),
                alt: Some(image.alt.clone()),
            });
        }
        self.html.push_str("</div>\n");
    }

    /// Writes an embed: an allowed provider's player, or a page on a domain
    /// the site allows, in a lazy, sandboxed frame of fixed size with the
    /// page's link under it, or the link alone.
    pub(super) fn embed(&mut self, src: &AbsoluteUrl, title: &str, height: Option<u16>) {
        self.link_out(src, title);
        let link = [
            "<a href=\"",
            &attribute(src.as_str()),
            "\">",
            &attribute(title),
            "</a>",
        ]
        .concat();
        let Some((provider, player)) = embed::player(src) else {
            if src.as_str().starts_with("https://")
                && self
                    .frames
                    .iter()
                    .any(|domain| domain.as_str() == src.host())
            {
                let height = height.unwrap_or(DEFAULT_HEIGHT).to_string();
                self.push(&[
                    "<figure class=\"embed embed-site\">\n<iframe src=\"",
                    &attribute(src.as_str()),
                    "\" title=\"",
                    &attribute(title),
                    "\" width=\"560\" height=\"",
                    &height,
                    "\" loading=\"lazy\" referrerpolicy=\"strict-origin-when-cross-origin\" sandbox=\"allow-scripts allow-same-origin allow-forms allow-popups\"></iframe>\n<figcaption>",
                    &link,
                    "</figcaption>\n</figure>\n",
                ]);
            } else {
                self.push(&["<p class=\"bookmark\">", &link, "</p>\n"]);
            }
            return;
        };
        let (shape, height) = if provider.is_audio() {
            ("audio", "152")
        } else {
            ("wide", "315")
        };
        let words = CardWords::of(&self.ids.locale);
        let name = provider.name();
        self.push(&[
            "<figure class=\"embed embed-",
            shape,
            "\">\n",
            MEDIA_CARD_START,
            "<div class=\"embed-card\">\n<p>",
            words.note[0],
            name,
            words.note[1],
            name,
            words.note[2],
            "</p>\n<div class=\"embed-card-actions\"><a class=\"embed-load\" href=\"",
            &attribute(src.as_str()),
            "\">",
            words.load,
            "</a>\n",
            MEDIA_FORM,
            "<button type=\"submit\" name=\"choice\" value=\"allow-external-media\">",
            words.always,
            "</button></form></div>\n</div>",
            MEDIA_CARD_END,
            "\n",
            MEDIA_START,
            "<iframe src=\"",
            &attribute(&player),
            "\" title=\"",
            &attribute(title),
            "\" width=\"560\" height=\"",
            height,
            "\" loading=\"lazy\" referrerpolicy=\"strict-origin-when-cross-origin\" sandbox=\"allow-scripts allow-same-origin allow-popups\" allow=\"encrypted-media; fullscreen; picture-in-picture\"></iframe>",
            MEDIA_END,
            "\n<figcaption>",
            &link,
            "</figcaption>\n</figure>\n",
        ]);
    }
}

/// The words on an embed's card, in German for a German post and in
/// English for every other.
struct CardWords {
    /// What loading shares, around the provider's name twice.
    note: [&'static str; 3],
    /// The link that loads this one player.
    load: &'static str,
    /// The button that allows External media for the whole site.
    always: &'static str,
}

impl CardWords {
    fn of(locale: &Locale) -> Self {
        if locale.language() == "de" {
            Self {
                note: [
                    "Dieser Player kommt von ",
                    ". Beim Laden erhält ",
                    " Daten über Sie, etwa Ihre IP-Adresse.",
                ],
                load: "Medien laden",
                always: "Externe Medien immer laden",
            }
        } else {
            Self {
                note: [
                    "This player comes from ",
                    ". Loading it shares data about you, such as your IP address, with ",
                    ".",
                ],
                load: "Load media",
                always: "Always load external media",
            }
        }
    }
}

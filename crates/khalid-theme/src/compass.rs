//! Compass (templates spec 3.4): a teaching blog where every guide opens
//! with its trail, its contents and its key points, and ends with share
//! links, its author and the newsletter. It has its own front page of
//! titled sections and keeps the shared post page. It
//! shows the author's name without a photo at the top, turns off reading
//! time and related posts. Its palette, fonts and calm type scale live in
//! `packs/compass/tokens.toml`.

use khalid_core::ReaderSwitch;

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{Pack, PackTemplate, PackText, slot};

/// The template files the compass pack replaces.
static COMPASS_TEMPLATES: [PackTemplate; 2] = [
    PackTemplate::file("picture_sizes.html", "compass/picture_sizes.html"),
    PackTemplate::file("home.html", "compass/home.html"),
];

/// The Compass pack.
pub(crate) static COMPASS: Pack = Pack {
    name: "compass",
    summary: "A teaching blog: breadcrumbs, the contents and key points before each guide, share links, the author and the newsletter after it.",
    css: PackText::file("compass/compass.css"),
    templates: &COMPASS_TEMPLATES,
    tokens: PackText::file("compass/tokens.toml"),
    reader: &[
        (ReaderSwitch::Byline, false),
        (ReaderSwitch::ReadingTime, false),
        (ReaderSwitch::RelatedPosts, false),
    ],
    options: &[
        parts::on(parts::LOGO),
        slot::CTA_BUTTON_ON,
        parts::on(parts::BREADCRUMBS),
        parts::cards(CardShape::Image),
        parts::pagination(PageLinks::Numbered),
        parts::contents(ContentsPlacement::Inline),
        parts::READING_PROGRESS,
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["linkedin", "x", "facebook", "email"]),
        slot::newsletter("both"),
        slot::cta_band("post"),
        parts::HERO_STATS,
        parts::on(parts::TOPIC_TABS),
        parts::on(parts::VIDEO_SECTION),
        parts::on(parts::PODCAST_SECTION),
    ],
};

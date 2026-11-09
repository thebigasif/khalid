//! Syllabus: an online course (docs/templates/syllabus.md). Lessons are
//! posts with facts, modules are categories, and the course is a series,
//! so every lesson shows "Lesson 5 of 13" and a progress ruler, one
//! square per lesson, with no script. The front page shows the course
//! map and the outline, module by module. Built from the shared section
//! parts in `khalid-render/templates/sections/`.

use crate::parts::{self, CardShape, ContentsNumbers, ContentsPlacement, PageLinks};
use crate::{OptionKind, OptionSpec, Pack, PackTemplate, PackText, slot};

/// The Syllabus pack.
pub(crate) static SYLLABUS: Pack = Pack {
    name: "syllabus",
    summary: "For an online course: modules and lessons in a course outline, a course map with one square per lesson, and on every lesson a progress ruler that says how far through the course it is.",
    css: PackText::file("syllabus/syllabus.css"),
    templates: &[
        PackTemplate::file("picture_sizes.html", "syllabus/picture_sizes.html"),
        PackTemplate::file("home.html", "syllabus/home.html"),
        PackTemplate::file("post.html", "syllabus/post.html"),
        PackTemplate::file("listing.html", "syllabus/listing.html"),
        PackTemplate::file("authors.html", "syllabus/authors.html"),
        PackTemplate::file("author.html", "syllabus/author.html"),
        PackTemplate::file("parts-lesson-rows.html", "syllabus/parts-lesson-rows.html"),
    ],
    tokens: PackText::file("syllabus/tokens.toml"),
    reader: &[],
    options: &[
        parts::on(parts::LOGO),
        parts::on(parts::BREADCRUMBS),
        parts::on(parts::DEK),
        parts::UPDATED_NOTE,
        parts::cards(CardShape::Text),
        parts::pagination(PageLinks::Numbered),
        parts::related_count(5),
        parts::contents(ContentsPlacement::InlineFolded),
        parts::HEADER_SEARCH,
        parts::contents_numbers(ContentsNumbers::Numbered),
        slot::share("end"),
        slot::share_networks(&["bluesky", "linkedin", "email"]),
        slot::newsletter("post"),
        slot::cta_band("home"),
        slot::CTA_BUTTON_OFF,
        OptionSpec {
            key: "tagline",
            help: "One line under the course's name at the top of the front page, such as what the course teaches and how long it takes.",
            kind: OptionKind::Lines { most: 1 },
        },
        OptionSpec {
            key: "start_button",
            help: "Whether the front page shows a button to the first lesson under the course's name.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "course_map",
            help: "Whether the front page shows the course map: every lesson as a square, grouped by module.",
            kind: OptionKind::Switch { default: true },
        },
        OptionSpec {
            key: "news_count",
            help: "How many course news posts, posts without facts, show beside the course outline on the front page.",
            kind: OptionKind::Count {
                least: 0,
                most: 10,
                default: 4,
            },
        },
        parts::HERO_STATS,
        parts::TOPIC_TABS,
        parts::VIDEO_SECTION,
        parts::PODCAST_SECTION,
    ],
};

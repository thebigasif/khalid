//! The home page's rows made from the posts: each topic's newest posts
//! for the topic tabs, and the newest posts with a video or a sound
//! player. A template shows each only when its option is on.

use std::collections::BTreeMap;

use khalid_render::{HomeSectionPosts, PageBody, PostCard};

use crate::build_pages::LanguagePages;

/// The most topics the tabs show: more would wrap or scroll on a phone.
const MOST_TABS: usize = 6;

/// How many posts a topic's tab shows: one row of four on a wide screen.
const PER_TAB: usize = 4;

/// How many posts a video or podcast row shows: one row of three.
const PER_ROW: usize = 3;

/// The markup a post's body has when it carries a video: the video block
/// or a player embedded from a video site.
const VIDEO_MARKS: [&str; 4] = [
    "<figure class=\"video\">",
    "src=\"https://www.youtube-nocookie.com/",
    "src=\"https://player.vimeo.com/",
    "src=\"https://www.loom.com/",
];

/// The markup a post's body has when it carries a sound player: the audio
/// block or a player embedded from a sound site.
const AUDIO_MARKS: [&str; 2] = [
    "<figure class=\"audio\">",
    "<figure class=\"embed embed-audio\">",
];

/// Fills the topic tabs and the video and podcast rows of each home page
/// of `languages` from the language's dated posts, newest first.
pub(crate) fn fill(languages: &mut [LanguagePages]) {
    for language in languages.iter_mut() {
        let cards: Vec<&PostCard> = language.listed.iter().map(|post| &post.card).collect();
        let tabs = tabs(&cards);
        let bodies: BTreeMap<&str, &str> = (language.pages.iter())
            .filter_map(|page| match &page.body {
                PageBody::Post(post) => Some((page.meta.path.as_str(), post.body.as_str())),
                _ => None,
            })
            .collect();
        let with = |marks: &[&str]| -> Vec<PostCard> {
            let carries = |html: &&str| marks.iter().any(|mark| html.contains(mark));
            (cards.iter())
                .filter(|card| bodies.get(card.href.as_str()).is_some_and(carries))
                .take(PER_ROW)
                .map(|card| (*card).clone())
                .collect()
        };
        let videos = with(&VIDEO_MARKS);
        let podcasts = with(&AUDIO_MARKS);
        for page in &mut language.pages {
            if let PageBody::Home(home) = &mut page.body {
                home.tabs.clone_from(&tabs);
                home.videos.clone_from(&videos);
                home.podcasts.clone_from(&podcasts);
            }
        }
    }
}

/// One tab per category of `cards`, in the order their newest post
/// comes, each with its newest posts. Categories are named by their
/// slug until `post_categories::fill` names them.
fn tabs(cards: &[&PostCard]) -> Vec<HomeSectionPosts> {
    let mut tabs: Vec<HomeSectionPosts> = Vec::new();
    for card in cards {
        let Some(category) = card.category.as_ref() else {
            continue;
        };
        let room = tabs.len() < MOST_TABS;
        match tabs.iter_mut().find(|tab| tab.href == category.href) {
            Some(tab) if tab.posts.len() < PER_TAB => tab.posts.push((*card).clone()),
            None if room => tabs.push(HomeSectionPosts {
                title: category.name.clone(),
                href: category.href.clone(),
                posts: vec![(*card).clone()],
            }),
            // A full tab, or a topic past the last tab, shows no more.
            _ => {}
        }
    }
    tabs
}

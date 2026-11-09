//! The home page's rows made from the posts: each topic's newest posts
//! for the topic tabs, the newest posts with a video or a sound player,
//! and the sound player of the newest of those. A template shows each
//! only when it reads it.

use std::collections::BTreeMap;

use khalid_render::{BodyHtml, HomeSectionPosts, PageBody, PostCard};

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

/// Where a sound player ends: its figure's closing tag.
const FIGURE_END: &str = "</figure>";

/// The attributes of a sound player that hold an address.
const ADDRESS_ATTRIBUTES: [&str; 2] = ["src=\"", "href=\""];

/// Fills the topic tabs, the video and podcast rows and the newest
/// episode's player of each home page of `languages` from the language's
/// dated posts, newest first.
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
        let player = podcasts.first().and_then(|card| {
            let html = bodies.get(card.href.as_str()).copied()?;
            let player = first_player(html)?;
            Some(BodyHtml::from_sanitised(under_post(player, &card.href)))
        });
        for page in &mut language.pages {
            if let PageBody::Home(home) = &mut page.body {
                home.tabs.clone_from(&tabs);
                home.videos.clone_from(&videos);
                home.podcasts.clone_from(&podcasts);
                home.podcast_player.clone_from(&player);
            }
        }
    }
}

/// The first sound player in a post's `html`: the figure an audio mark
/// opens, up to its closing tag. `None` when the post carries none. The
/// body was sanitised, so the figure is too.
fn first_player(html: &str) -> Option<&str> {
    let start = AUDIO_MARKS
        .iter()
        .filter_map(|mark| html.find(mark))
        .min()?;
    let rest = html.get(start..)?;
    let end = rest.find(FIGURE_END)?.saturating_add(FIGURE_END.len());
    rest.get(..end)
}

/// `player` with each relative address made a path from the folder of the
/// post at `post`, since the home page does not sit beside the post's
/// files: `episode.mp3` on `/blog/episode-one/` becomes
/// `/blog/episode-one/episode.mp3`. Full addresses, paths from the root
/// and anchors stay as they are.
fn under_post(player: &str, post: &str) -> String {
    let folder = post
        .rfind('/')
        .and_then(|at| post.get(..=at))
        .unwrap_or("/");
    let mut out = String::with_capacity(player.len().saturating_add(folder.len()));
    let mut rest = player;
    while let Some((at, length)) = (ADDRESS_ATTRIBUTES.iter())
        .filter_map(|attribute| rest.find(attribute).map(|at| (at, attribute.len())))
        .min()
    {
        let value_start = at.saturating_add(length);
        out.push_str(rest.get(..value_start).unwrap_or_default());
        rest = rest.get(value_start..).unwrap_or_default();
        let value = rest.split('"').next().unwrap_or_default();
        let relative = !(value.starts_with('/') || value.starts_with('#') || value.contains(':'));
        if relative {
            out.push_str(folder);
        }
    }
    out.push_str(rest);
    out
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

#[cfg(test)]
mod tests {
    use super::*;

    const POST: &str = "<p>Notes.</p>\n<figure class=\"audio\">\n<figcaption>Episode one</figcaption>\n<audio controls preload=\"none\" src=\"episode-1.mp3\"><a href=\"episode-1.mp3\">Episode one</a></audio>\n</figure>\n<p>More notes.</p>\n<figure class=\"audio\">\n</figure>\n";

    #[test]
    fn the_player_is_the_first_sound_figure_of_the_post() {
        let player = first_player(POST).unwrap();
        assert!(player.starts_with("<figure class=\"audio\">"), "{player}");
        assert!(player.ends_with("</audio>\n</figure>"), "{player}");
        assert!(!player.contains("More notes"), "{player}");
    }

    #[test]
    fn an_embedded_player_counts_and_a_post_without_one_has_none() {
        let embed = "<p>Hi</p><figure class=\"embed embed-audio\"><iframe src=\"https://open.spotify.com/embed/episode/1\" title=\"Episode\"></iframe></figure>";
        let player = first_player(embed).unwrap();
        assert!(player.starts_with("<figure class=\"embed embed-audio\">"));
        assert_eq!(first_player("<p>No sound here.</p>"), None);
        assert_eq!(first_player("<figure class=\"audio\"><audio"), None);
    }

    #[test]
    fn a_relative_sound_file_plays_from_the_home_page() {
        let player = first_player(POST).unwrap();
        let moved = under_post(player, "/blog/episode-one/");
        assert!(
            moved.contains("src=\"/blog/episode-one/episode-1.mp3\""),
            "{moved}"
        );
        assert!(
            moved.contains("href=\"/blog/episode-one/episode-1.mp3\""),
            "{moved}"
        );
    }

    #[test]
    fn full_addresses_root_paths_and_anchors_stay_as_they_are() {
        let player = "<audio src=\"https://cdn.example.com/a.mp3\"><a href=\"/media/a.mp3\">A</a><a href=\"#notes\">Notes</a></audio>";
        assert_eq!(under_post(player, "/blog/episode-one/"), player);
    }
}

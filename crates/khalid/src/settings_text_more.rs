//! A post's ties to other posts and its listing choices in its front
//! matter: read for the Admin API, and written back field by field.

use khalid_api::resources::{
    ClusterMembership, ClusterPlace, NonBlank, PostConnections, PostListing, SeriesPart,
};
use khalid_content::{self as content, PostFront, Sitemap, TopicRole};
use khalid_core::Slug;

use super::{group, list, put};
use crate::post_text::{PostText, quoted};

/// The ties to other posts that `front` holds.
pub(super) fn connections_of(front: &PostFront) -> PostConnections {
    let mut connections = PostConnections::default();
    connections.translation = front.translation.as_deref().and_then(NonBlank::try_new);
    connections.related.clone_from(&front.related);
    connections.series = front.series.as_ref().and_then(series_part);
    connections.cluster = front.cluster.as_ref().map(|cluster| {
        let role = match cluster.role {
            TopicRole::Pillar => ClusterPlace::Pillar,
            _ => ClusterPlace::Member,
        };
        ClusterMembership::new(cluster.name.clone(), role)
    });
    connections.topic_group = front.topic_group.as_deref().and_then(NonBlank::try_new);
    connections.main_post = front.main_post;
    connections
}

/// The series part in the API's shape; `None` for a blank name, which
/// front matter would not hold.
fn series_part(series: &content::PostSeries) -> Option<SeriesPart> {
    let name = NonBlank::try_new(&series.name)?;
    Some(SeriesPart::new(name, series.order))
}

/// The listing choices that `front` holds.
pub(super) fn listing_of(front: &PostFront) -> PostListing {
    let mut listing = PostListing::default();
    listing.sitemap = front.sitemap != Sitemap::Unlisted;
    listing.news = front.news;
    let breadcrumb = front.breadcrumb_title.as_deref();
    listing.breadcrumb_title = breadcrumb.and_then(NonBlank::try_new);
    listing.page_type = front.page_type;
    listing.article_type = front.article_type;
    let synonyms = front.keyword_synonyms.iter();
    listing.keyword_synonyms = synonyms
        .filter_map(|word| NonBlank::try_new(word))
        .collect();
    listing.reviewed_by.clone_from(&front.reviewed_by);
    listing
}

/// Writes the ties that differ between `before` and `after`.
pub(super) fn write_connections(
    text: &mut PostText,
    before: &PostConnections,
    after: &PostConnections,
) {
    if after.translation != before.translation {
        let key = after.translation.as_ref().map(|key| quoted(key.as_str()));
        put(text, "translation", key);
    }
    if after.related != before.related {
        let slugs: Vec<&str> = after.related.iter().map(Slug::as_str).collect();
        put(text, "related", list(&slugs));
    }
    if after.series != before.series {
        let series = after.series.as_ref().and_then(|part| {
            group(&[
                ("name", Some(quoted(part.name.as_str()))),
                ("order", part.order.map(|order| order.to_string())),
            ])
        });
        put(text, "series", series);
    }
    if after.cluster != before.cluster {
        let cluster = after.cluster.as_ref().and_then(|cluster| {
            let role = match cluster.role {
                ClusterPlace::Pillar => "pillar",
                ClusterPlace::Member => "member",
            };
            group(&[
                ("name", Some(cluster.name.to_string())),
                ("role", Some(role.to_owned())),
            ])
        });
        put(text, "cluster", cluster);
    }
    if after.topic_group != before.topic_group {
        let name = after.topic_group.as_ref().map(|name| quoted(name.as_str()));
        put(text, "topic_group", name);
    }
    if after.main_post != before.main_post {
        put(
            text,
            "main_post",
            after.main_post.then(|| "true".to_owned()),
        );
    }
}

/// Writes the listing choices that differ between `before` and `after`.
/// A listed post, the default, is written as no `sitemap` line at all.
pub(super) fn write_listing(text: &mut PostText, before: &PostListing, after: &PostListing) {
    if after.sitemap != before.sitemap {
        put(
            text,
            "sitemap",
            (!after.sitemap).then(|| "false".to_owned()),
        );
    }
    if after.news != before.news {
        put(text, "news", after.news.map(|news| news.to_string()));
    }
    if after.breadcrumb_title != before.breadcrumb_title {
        let title = after.breadcrumb_title.as_ref().map(|t| quoted(t.as_str()));
        put(text, "breadcrumb_title", title);
    }
    if after.page_type != before.page_type {
        let kind = after.page_type.map(|kind| kind.as_str().to_owned());
        put(text, "page_type", kind);
    }
    if after.article_type != before.article_type {
        let kind = after.article_type.map(|kind| kind.as_str().to_owned());
        put(text, "article_type", kind);
    }
    if after.keyword_synonyms != before.keyword_synonyms {
        let words: Vec<String> = (after.keyword_synonyms.iter())
            .map(|word| quoted(word.as_str()))
            .collect();
        let words: Vec<&str> = words.iter().map(String::as_str).collect();
        put(text, "keyword_synonyms", list(&words));
    }
    if after.reviewed_by != before.reviewed_by {
        let slugs: Vec<&str> = after.reviewed_by.iter().map(Slug::as_str).collect();
        put(text, "reviewed_by", list(&slugs));
    }
}

//! A post's ties to other posts in its front matter: read for the
//! Admin API, and written back field by field.

use khalid_api::resources::{
    ClusterMembership, ClusterPlace, NonBlank, PostConnections, SeriesPart,
};
use khalid_content::{self as content, PostFront, TopicRole};
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

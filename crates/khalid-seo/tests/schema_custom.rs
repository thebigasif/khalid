//! The custom JSON-LD field per post and per site (task 1.14, KH-SEO-3).

#![cfg(test)]

mod common;

use common::{demo_site, internal_links_post, internal_links_post_de};
use khalid_seo::{CustomSchema, SchemaGraph, SchemaNode, SchemaValue, SeoError, page_graph};

const EVENT: &str = r#"{
  "@context": "https://schema.org",
  "@type": "Event",
  "name": "Internal links clinic",
  "startDate": "2026-11-12T18:00:00+00:00",
  "eventAttendanceMode": "https://schema.org/OnlineEventAttendanceMode",
  "isAccessibleForFree": true,
  "maximumAttendeeCapacity": 40,
  "location": {
    "@type": "VirtualLocation",
    "url": "https://www.example.com/blog/clinic/"
  },
  "organizer": { "@id": "https://www.example.com/blog/#organization" }
}"#;

#[test]
fn a_post_s_custom_json_ld_joins_its_graph() {
    let mut post = internal_links_post();
    post.custom_schema = CustomSchema::parse(EVENT).unwrap();
    let graph = page_graph(&demo_site(), &post).unwrap();
    let node = graph.nodes().last().unwrap().clone();
    assert_eq!(node.kind(), "Event");
    assert_eq!(
        node.get("maximumAttendeeCapacity"),
        Some(&SchemaValue::Integer(40))
    );
    insta::assert_snapshot!(SchemaGraph::new(vec![node]).to_json());
}

#[test]
fn a_graph_gives_one_node_each_and_the_site_s_come_before_the_post_s() {
    let mut site = demo_site();
    site.custom_schema = CustomSchema::parse(
        r#"{"@context": "https://schema.org", "@graph": [
            {"@type": "Course", "name": "Links for beginners"},
            {"@type": "Book", "name": "The internal links handbook"}
        ]}"#,
    )
    .unwrap();
    let mut post = internal_links_post_de();
    post.custom_schema = CustomSchema::parse(EVENT).unwrap();
    let graph = page_graph(&site, &post).unwrap();
    let kinds: Vec<&str> = graph.nodes().iter().map(SchemaNode::kind).collect();
    assert_eq!(
        kinds.get(kinds.len() - 3..),
        Some(&["Course", "Book", "Event"][..])
    );
}

#[test]
fn a_script_in_custom_text_cannot_close_the_tag() {
    let mut post = internal_links_post();
    post.custom_schema =
        CustomSchema::parse(r#"{"@type": "Course", "name": "</script><script>alert(1)"}"#).unwrap();
    let tag = page_graph(&demo_site(), &post).unwrap().to_script_tag();
    assert_eq!(tag.matches("</script>").count(), 1, "{tag}");
}

#[test]
fn an_empty_field_adds_nothing() {
    assert_eq!(CustomSchema::parse("  \n").unwrap().nodes(), &[]);
    assert_eq!(CustomSchema::default().nodes(), &[]);
}

#[test]
fn broken_json_says_where() {
    assert_eq!(
        CustomSchema::parse("{\n  \"@type\": \"Event\",\n}"),
        Err(SeoError::CustomSchemaSyntax { line: 3, column: 1 })
    );
}

#[test]
fn each_refused_shape_names_the_place_and_the_problem() {
    let cases = [
        ("[]", ""),
        (r#""Event""#, ""),
        (r#"{"name": "No type"}"#, ""),
        (r#"{"@type": ["Event", "Thing"]}"#, "@type"),
        (r#"{"@type": "event"}"#, "@type"),
        (
            r#"{"@context": "https://example.com", "@type": "Event"}"#,
            "@context",
        ),
        (r#"{"@type": "Event", "@id": "/clinic/"}"#, "@id"),
        (r#"{"@type": "Event", "@reverse": {}}"#, "@reverse"),
        (r#"{"@type": "Event", "Name": "Clinic"}"#, "Name"),
        (r#"{"@type": "Event", "name": null}"#, "name"),
        (
            r#"{"@type": "Event", "location": {"name": "Room 2"}}"#,
            "location",
        ),
        (r#"{"@graph": [{"@type": "Event"}, 3]}"#, "@graph[1]"),
        (
            r#"{"@type": "Event", "location": {"@type": "Place", "@context": "https://schema.org"}}"#,
            "location.@context",
        ),
    ];
    for (text, place) in cases {
        let result = CustomSchema::parse(text);
        let Err(SeoError::CustomSchema { at, .. }) = &result else {
            panic!("{text} gave {result:?}");
        };
        assert_eq!(at, place, "{text}");
    }
}

#[test]
fn a_slip_in_a_type_or_property_name_says_which_name_was_meant() {
    let cases = [
        (r#"{"@type": "Eevnt"}"#, "@type", "Eevnt", "Event"),
        (
            r#"{"@type": "Organisation"}"#,
            "@type",
            "Organisation",
            "Organization",
        ),
        (r#"{"@type": "Faqpage"}"#, "@type", "Faqpage", "FAQPage"),
        (
            r#"{"@type": "Event", "startdate": "2026-11-12"}"#,
            "startdate",
            "startdate",
            "startDate",
        ),
        (
            r#"{"@type": "Event", "organiser": {"@type": "Person", "name": "Asif"}}"#,
            "organiser",
            "organiser",
            "organizer",
        ),
        (
            r#"{"@type": "Book", "authors": {"@type": "Person", "name": "Asif"}}"#,
            "authors",
            "authors",
            "author",
        ),
        (
            r#"{"@type": "Event", "location": {"@type": "Plaec", "name": "Room 2"}}"#,
            "location.@type",
            "Plaec",
            "Place",
        ),
        (
            r#"{"@graph": [{"@type": "Course", "providr": {"@id": "https://example.com/#o"}}]}"#,
            "@graph[0].providr",
            "providr",
            "provider",
        ),
        (
            r#"{"@type": "Event", "maximumAttendeCapacity": 40}"#,
            "maximumAttendeCapacity",
            "maximumAttendeCapacity",
            "maximumAttendeeCapacity",
        ),
    ];
    for (text, place, written, meant) in cases {
        assert_eq!(
            CustomSchema::parse(text),
            Err(SeoError::SchemaNameSlip {
                at: place.to_owned(),
                name: written.to_owned(),
                known: meant,
            }),
            "{text}"
        );
    }
}

#[test]
fn a_slip_message_names_the_place_and_the_fix() {
    let error = CustomSchema::parse(r#"{"@type": "Event", "startdate": "2026-11-12"}"#)
        .unwrap_err()
        .to_string();
    assert_eq!(
        error,
        "in the custom JSON-LD, startdate is not a schema.org name; did you mean startDate?"
    );
}

#[test]
fn names_far_from_any_known_one_and_real_near_pairs_still_pass() {
    // Rare types and properties the table does not hold pass, and so do real
    // names one letter apart from each other.
    let texts = [
        r#"{"@type": "TouristAttraction", "touristType": "families"}"#,
        r#"{"@type": "Resort", "name": "Sea view"}"#,
        r#"{"@type": "Report", "name": "Annual"}"#,
        r#"{"@type": "Product", "name": "Box", "height": "2 cm", "weight": "1 kg"}"#,
        r#"{"@type": "Book", "name": "Links", "isbn": "9780000000002", "issn": "0000-0000"}"#,
        r#"{"@type": "HowTo", "name": "Fix", "step": "Open it", "steps": "Open it"}"#,
        r#"{"@type": "Event", "name": "Gig", "performer": "A", "performers": "A"}"#,
        r#"{"@type": "Product", "name": "Box", "gtin12": "000000000000", "gtin13": "0000000000000"}"#,
    ];
    for text in texts {
        assert!(CustomSchema::parse(text).is_ok(), "{text}");
    }
}

#[test]
fn every_graph_khalid_writes_reads_back_as_custom_json_ld() {
    let mut site = demo_site();
    site.custom_schema = CustomSchema::parse(EVENT).unwrap();
    for post in [internal_links_post(), internal_links_post_de()] {
        let json = page_graph(&site, &post).unwrap().to_json();
        let read = CustomSchema::parse(&json);
        assert!(read.is_ok(), "{read:?}");
    }
}

#[test]
fn no_name_in_a_snapshot_graph_reads_as_a_slip() {
    let folder = format!("{}/tests/snapshots", env!("CARGO_MANIFEST_DIR"));
    let mut graphs = 0;
    for entry in std::fs::read_dir(folder).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let Some((_, body)) = text.split_once("\n---\n") else {
            continue;
        };
        if !body.trim_start().starts_with('{') || !body.contains("\"@type\"") {
            continue;
        }
        graphs += 1;
        let read = CustomSchema::parse(body);
        assert!(
            !matches!(read, Err(SeoError::SchemaNameSlip { .. })),
            "{read:?}"
        );
    }
    assert!(graphs > 10, "{graphs}");
}

#[test]
fn a_field_over_64_kb_is_refused() {
    let long = format!(
        r#"{{"@type": "Course", "name": "{}"}}"#,
        "a".repeat(64 * 1024)
    );
    assert!(matches!(
        CustomSchema::parse(&long),
        Err(SeoError::CustomSchema { .. })
    ));
}

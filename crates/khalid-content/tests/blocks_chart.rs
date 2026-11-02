//! The chart block, `:::chart`, numbers written in the post or in a CSV
//! file beside it (engagement task, part 1).

use khalid_content::{BlockKind, ChartBlock, ChartKind, ChartTable, parse_blocks, print_blocks};

#[cfg(test)]
fn chart(markdown: &str) -> ChartBlock {
    let blocks = parse_blocks(markdown).unwrap();
    assert_eq!(print_blocks(&blocks), markdown);
    match &blocks[..] {
        [only] => match &only.kind {
            BlockKind::Chart(chart) => chart.clone(),
            other => panic!("{markdown:?} gave {other:?}"),
        },
        other => panic!("{markdown:?} gave {other:?}"),
    }
}

#[cfg(test)]
fn error(markdown: &str) -> String {
    parse_blocks(markdown).unwrap_err().to_string()
}

const VISITS: &str = ":::chart{type=\"line\" title=\"Visits from search\" unit=\"k\"}\nMonth, Google, Bing\nJan, 12.5, 1\nFeb, 14, 1.5\n\nMar, 17, -2\n:::\n";

#[test]
fn a_chart_round_trips_with_its_series_and_rows() {
    let chart = chart(VISITS);
    assert_eq!(chart.kind, ChartKind::Line);
    assert_eq!(chart.title, "Visits from search");
    assert_eq!(chart.unit.as_deref(), Some("k"));
    assert_eq!(chart.src, None);
    assert_eq!(chart.table.label, "Month");
    assert_eq!(chart.table.series, ["Google", "Bing"]);
    let labels: Vec<&str> = chart.table.rows.iter().map(|r| r.label.as_str()).collect();
    assert_eq!(labels, ["Jan", "Feb", "Mar"]);
    let mar: Vec<f64> = chart.table.rows[2]
        .values
        .iter()
        .map(khalid_content::ChartValue::number)
        .collect();
    assert_eq!(mar, [17.0, -2.0]);
    assert_eq!(chart.table.rows[0].values[0].as_str(), "12.5");
}

#[test]
fn a_chart_is_a_column_chart_unless_it_says_and_reads_quoted_cells() {
    let chart =
        chart(":::chart{title=\"Posts per writer\"}\nWriter, Posts\n\"Maya, editor\", 4\n:::\n");
    assert_eq!(chart.kind, ChartKind::Column);
    assert_eq!(chart.table.rows[0].label, "Maya, editor");
    let bar = self::chart(":::chart{type=\"bar\" title=\"T\"}\nA, B\nx, 1\n:::\n");
    assert_eq!(bar.kind, ChartKind::Bar);
    assert_eq!(bar.kind.name(), "bar");
}

#[test]
fn a_chart_from_a_file_has_no_rows_until_the_build_reads_it() {
    let chart = chart(":::chart{type=\"bar\" title=\"Traffic\" src=\"traffic.csv\"}\n:::\n");
    assert_eq!(chart.src.as_deref(), Some("traffic.csv"));
    assert_eq!(chart.table, ChartTable::default());
}

#[test]
fn a_wrong_chart_says_how_to_write_it_on_its_line() {
    assert_eq!(
        error(":::chart{title=\"T\"}\n:::\n"),
        "line 1: a chart needs numbers: a header line and one line per row between its ::: lines, or src, a CSV file beside the post"
    );
    assert_eq!(
        error(":::chart{type=\"pie\" title=\"T\"}\nA, B\nx, 1\n:::\n"),
        "line 1: a chart's type is bar, column or line"
    );
    assert!(error(":::chart{type=\"bar\"}\nA, B\nx, 1\n:::\n").contains("a chart needs title"));
    assert!(error(":::chart{title=\"T\"}\nA, B\nx, 1,200\n:::\n").starts_with("line 3: each line"));
    assert!(
        error(":::chart{title=\"T\"}\nA, B\nx, 12%\n:::\n").starts_with("line 3: a chart's values")
    );
    assert!(
        error(":::chart{title=\"T\"}\nA\nx\n:::\n").starts_with("line 2: a chart's first line")
    );
    assert!(error(":::chart{title=\"T\"}\nA, B\n:::\n").contains("one to 40 rows"));
    assert!(error(":::chart{title=\"T\" src=\"/etc/x.csv\"}\n:::\n").contains("beside the post"));
    assert!(
        error(":::chart{title=\"T\" colour=\"red\"}\nA, B\nx, 1\n:::\n")
            .contains("takes type, title, unit and src")
    );
    assert!(error("::chart{title=\"T\"}\n").contains("open it with :::chart"));
}

#[test]
fn a_chart_takes_at_most_four_series_and_forty_rows() {
    assert!(
        error(":::chart{title=\"T\"}\nA, 1, 2, 3, 4, 5\nx, 1, 2, 3, 4, 5\n:::\n")
            .contains("first line")
    );
    let rows: Vec<String> = (0..41).map(|n| format!("r{n}, {n}")).collect();
    let rows = rows.join("\n");
    assert!(
        error(&format!(":::chart{{title=\"T\"}}\nA, B\n{rows}\n:::\n")).contains("one to 40 rows")
    );
}

#[cfg(test)]
fn html(markdown: &str) -> String {
    let en = khalid_core::Locale::try_new("en").unwrap();
    khalid_content::render(markdown, &en, khalid_core::SlugMode::Ascii)
        .unwrap()
        .html
}

#[test]
fn a_bar_chart_puts_each_label_over_its_bar_and_the_value_after_it() {
    let html = html(
        ":::chart{type=\"bar\" title=\"Time to first click\" unit=\"seconds\"}\nPage, Time\nHome, 2\n\"Pricing <new>\", 4\n:::\n",
    );
    assert!(
        html.contains("<figure class=\"chart chart-bar\">"),
        "{html}"
    );
    assert!(
        !html.contains("chart-legend"),
        "one series needs no legend: {html}"
    );
    assert!(html.contains(">Pricing &lt;new&gt;</text>"), "{html}");
    assert!(
        html.contains("width=\"78.00%\" height=\"18\""),
        "the longest bar: {html}"
    );
    assert!(html.contains(">4 seconds</text>"), "{html}");
    insta::assert_snapshot!(html);
}

#[test]
fn a_chart_whose_file_was_not_found_keeps_its_caption_and_file_link() {
    let html = html(":::chart{title=\"Traffic\" src=\"traffic.csv\"}\n:::\n");
    assert_eq!(
        html,
        "<figure class=\"chart chart-column\">\n<figcaption>Traffic <a class=\"chart-file\" href=\"traffic.csv\" download>CSV</a></figcaption>\n</figure>\n"
    );
}

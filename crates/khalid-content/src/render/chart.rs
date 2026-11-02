//! The chart block as a picture drawn at build time: an SVG whose widths
//! are shares of the column, so it fits any screen with text at its own
//! size, and a table of the same numbers for screen readers. The colours
//! are classes the stylesheet fills from the theme. No script.

use std::fmt::Write as _;

use super::{Renderer, attribute, chart_plot};
use crate::{ChartBlock, ChartKind, ChartTable, ChartValue};

/// The share of the width the longest bar takes, leaving room for its value.
const BAR_REACH: f64 = 78.0;

impl Renderer<'_> {
    /// Writes a chart: its caption, the series' names when there are
    /// several, the drawing, the table and the link to its file.
    pub(super) fn chart(&mut self, chart: &ChartBlock) {
        let kind = chart.kind.name();
        let title = attribute(&chart.title);
        self.push(&[
            "<figure class=\"chart chart-",
            kind,
            "\">\n<figcaption>",
            &title,
        ]);
        if let Some(src) = &chart.src {
            let src = attribute(src);
            self.push(&[
                " <a class=\"chart-file\" href=\"",
                &src,
                "\" download>CSV</a>",
            ]);
        }
        self.html.push_str("</figcaption>\n");
        let table = &chart.table;
        if !table.rows.is_empty() {
            if table.series.len() > 1 {
                self.html
                    .push_str("<ul class=\"chart-legend\" aria-hidden=\"true\">");
                for (index, name) in table.series.iter().enumerate() {
                    let class = series_class(index);
                    self.push(&[
                        "<li><span class=\"",
                        class,
                        "\"></span>",
                        &attribute(name),
                        "</li>",
                    ]);
                }
                self.html.push_str("</ul>\n");
            }
            let unit = chart.unit.as_deref().unwrap_or_default();
            let drawing = match chart.kind {
                ChartKind::Bar => bars(table, unit),
                ChartKind::Column | ChartKind::Line => chart_plot::plot(chart.kind, table, unit),
            };
            self.html.push_str(&drawing);
            self.html.push_str(&numbers(&title, table, unit));
        }
        self.html.push_str("</figure>\n");
    }
}

/// The class that colours series `index`.
pub(super) fn series_class(index: usize) -> &'static str {
    ["s1", "s2", "s3", "s4"].get(index).copied().unwrap_or("s1")
}

/// `value` with `unit` after it, with a space before a word such as
/// `visits` and none before a sign such as `%` or `k`.
pub(super) fn with_unit(value: &str, unit: &str) -> String {
    if unit.chars().count() > 2 {
        format!("{value} {unit}")
    } else {
        format!("{value}{unit}")
    }
}

/// The lowest and highest value to draw, with zero always between them.
pub(super) fn range(table: &ChartTable) -> (f64, f64) {
    let values = table
        .rows
        .iter()
        .flat_map(|row| &row.values)
        .map(ChartValue::number);
    values.fold((0.0, 0.0), |(low, high), v| (v.min(low), v.max(high)))
}

/// The share of the width or height, from 0 to `reach`, that `value`
/// takes between `low` and `high`.
pub(super) fn share(value: f64, (low, high): (f64, f64), reach: f64) -> f64 {
    let span = high - low;
    if span <= 0.0 {
        0.0
    } else {
        (value - low) / span * reach
    }
}

/// Bars across: each row's label over its bars, one per series, with the
/// value at the end of each.
fn bars(table: &ChartTable, unit: &str) -> String {
    let series = table.series.len();
    let bar = if series > 1 { 12.0 } else { 18.0 };
    let row_height = 22.0 + bar * count(series) + 10.0;
    let height = row_height * count(table.rows.len());
    let range = range(table);
    let zero = share(0.0, range, BAR_REACH);
    let mut svg = String::new();
    let _ = write!(
        svg,
        "<svg class=\"chart-drawing\" width=\"100%\" height=\"{height}\" aria-hidden=\"true\">"
    );
    let mut top = 0.0;
    for row in &table.rows {
        let _ = write!(
            svg,
            "<text class=\"chart-label\" x=\"0\" y=\"{}\">{}</text>",
            top + 15.0,
            attribute(&row.label)
        );
        let mut y = top + 22.0;
        for (index, value) in row.values.iter().enumerate() {
            let end = share(value.number(), range, BAR_REACH);
            let (x, width) = if end < zero {
                (end, zero - end)
            } else {
                (zero, end - zero)
            };
            let _ = write!(
                svg,
                "<rect class=\"{}\" x=\"{x:.2}%\" y=\"{y}\" width=\"{width:.2}%\" height=\"{bar}\" rx=\"3\"></rect><text class=\"chart-value\" x=\"{:.2}%\" dx=\"6\" y=\"{}\">{}</text>",
                series_class(index),
                x + width,
                y + bar / 2.0 + 4.5,
                attribute(&with_unit(value.as_str(), unit)),
            );
            y += bar + 2.0;
        }
        top += row_height;
    }
    svg.push_str("</svg>\n");
    svg
}

/// `n` as a float, for the shares of the width rows and series take.
pub(super) fn count(n: usize) -> f64 {
    u32::try_from(n).map_or(0.0, f64::from)
}

/// The numbers as a table only screen readers see, named like the chart.
fn numbers(title: &str, table: &ChartTable, unit: &str) -> String {
    let mut html = format!(
        "<table class=\"visually-hidden\">\n<caption>{title}</caption>\n<thead><tr><th scope=\"col\">{}</th>",
        attribute(&table.label)
    );
    for name in &table.series {
        let _ = write!(html, "<th scope=\"col\">{}</th>", attribute(name));
    }
    html.push_str("</tr></thead>\n<tbody>\n");
    for row in &table.rows {
        let _ = write!(html, "<tr><th scope=\"row\">{}</th>", attribute(&row.label));
        for value in &row.values {
            let _ = write!(
                html,
                "<td>{}</td>",
                attribute(&with_unit(value.as_str(), unit))
            );
        }
        html.push_str("</tr>\n");
    }
    html.push_str("</tbody>\n</table>\n");
    html
}

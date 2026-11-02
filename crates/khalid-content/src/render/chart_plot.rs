//! Column and line charts: a plot with round gridlines, each labelled at
//! its start, and the rows' labels under it.

use std::fmt::Write as _;

use super::attribute;
use super::chart::{count, range, series_class, share, with_unit};
use crate::{ChartKind, ChartTable};

/// The height of the plot, in pixels.
const PLOT: f64 = 220.0;
/// The room above the plot for the top tick's label.
const TOP: f64 = 20.0;
/// The room under the plot for the row labels.
const UNDER: f64 = 28.0;

/// Columns upwards or lines across a plot with gridlines, each labelled
/// at its start, and the rows' labels under it.
pub(super) fn plot(kind: ChartKind, table: &ChartTable, unit: &str) -> String {
    let (step, ticks) = ticks(range(table));
    let range = (
        ticks.first().copied().unwrap_or(0.0),
        ticks.last().copied().unwrap_or(1.0),
    );
    let y = |value: f64| TOP + PLOT - share(value, range, PLOT);
    let height = TOP + PLOT + UNDER;
    let mut svg = String::new();
    let _ = write!(
        svg,
        "<svg class=\"chart-drawing\" width=\"100%\" height=\"{height}\" aria-hidden=\"true\">"
    );
    // The tick labels go on top of the marks, so a column never hides one.
    let mut labels = String::new();
    for tick in &ticks {
        let at = y(*tick);
        let line = if tick.abs() < f64::EPSILON {
            "chart-zero"
        } else {
            "chart-grid"
        };
        let _ = write!(
            svg,
            "<line class=\"{line}\" x1=\"0\" x2=\"100%\" y1=\"{at}\" y2=\"{at}\"></line>"
        );
        let _ = write!(
            labels,
            "<text class=\"chart-tick\" x=\"0\" y=\"{}\">{}</text>",
            at - 5.0,
            attribute(&with_unit(&number(*tick, step), unit)),
        );
    }
    let rows = table.rows.len();
    let slot = 100.0 / count(rows).max(1.0);
    // Every label shows when there is room; otherwise every few.
    let every = rows.div_ceil(8).max(1);
    for (index, row) in table
        .rows
        .iter()
        .enumerate()
        .filter(|(i, _)| i % every == 0)
    {
        let centre = slot * (count(index) + 0.5);
        let _ = write!(
            svg,
            "<text class=\"chart-label\" x=\"{centre:.2}%\" y=\"{}\" text-anchor=\"middle\">{}</text>",
            height - 8.0,
            attribute(&row.label)
        );
    }
    if kind == ChartKind::Line {
        lines(&mut svg, table, slot, &y);
    } else {
        columns(&mut svg, table, slot, &y);
    }
    svg.push_str(&labels);
    svg.push_str("</svg>\n");
    svg
}

/// One column per value, side by side within its row's slot.
fn columns(svg: &mut String, table: &ChartTable, slot: f64, y: &dyn Fn(f64) -> f64) {
    let series = count(table.series.len()).max(1.0);
    let width = slot * 0.64 / series;
    for (index, row) in table.rows.iter().enumerate() {
        let start = slot * count(index) + slot * 0.18;
        for (nth, value) in row.values.iter().enumerate() {
            let x = start + width * count(nth);
            let (top, bottom) = (y(value.number()), y(0.0));
            let (top, height) = if top < bottom {
                (top, bottom - top)
            } else {
                (bottom, top - bottom)
            };
            let _ = write!(
                svg,
                "<rect class=\"{}\" x=\"{x:.2}%\" y=\"{top:.1}\" width=\"{:.2}%\" height=\"{height:.1}\" rx=\"2\"></rect>",
                series_class(nth),
                width * 0.92
            );
        }
    }
}

/// One line per series through a dot at each row's value.
fn lines(svg: &mut String, table: &ChartTable, slot: f64, y: &dyn Fn(f64) -> f64) {
    for nth in 0..table.series.len() {
        let points: Vec<(f64, f64)> = (table.rows.iter().enumerate())
            .filter_map(|(index, row)| {
                let value = row.values.get(nth)?.number();
                Some((slot * (count(index) + 0.5), y(value)))
            })
            .collect();
        let class = series_class(nth);
        for pair in points.windows(2) {
            if let [(x1, y1), (x2, y2)] = pair {
                let _ = write!(
                    svg,
                    "<line class=\"chart-trace {class}\" x1=\"{x1:.2}%\" y1=\"{y1:.1}\" x2=\"{x2:.2}%\" y2=\"{y2:.1}\"></line>"
                );
            }
        }
        for (x, y) in &points {
            let _ = write!(
                svg,
                "<circle class=\"{class}\" cx=\"{x:.2}%\" cy=\"{y:.1}\" r=\"4\"></circle>"
            );
        }
    }
}

/// A round step and the gridlines it gives from below `low` to above
/// `high`, four or five of them.
fn ticks((low, high): (f64, f64)) -> (f64, Vec<f64>) {
    let span = (high - low).max(f64::EPSILON);
    let rough = span / 4.0;
    let magnitude = 10f64.powf(rough.log10().floor());
    let step = [1.0, 2.0, 2.5, 5.0, 10.0]
        .iter()
        .map(|m| m * magnitude)
        .find(|step| *step >= rough)
        .unwrap_or(magnitude * 10.0);
    let first = (low / step).floor();
    let last = (high / step).ceil().max(first + 1.0);
    let ticks = (0..=20)
        .map(|n| (first + f64::from(n)) * step)
        .take_while(|tick| *tick <= last * step + step / 2.0)
        .collect();
    (step, ticks)
}

/// `value` written with as many decimals as `step` needs, at most four.
fn number(value: f64, step: f64) -> String {
    let decimals = (0..4_u8)
        .find(|places| {
            let scaled = step * 10f64.powi(i32::from(*places));
            (scaled - scaled.round()).abs() < 1e-9
        })
        .unwrap_or(4);
    let decimals = usize::from(decimals);
    format!("{value:.decimals$}")
}

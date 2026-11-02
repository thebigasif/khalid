//! The chart block: numbers written in the post, or in a CSV file beside
//! it, which the build draws as a picture with no script.

/// The most rows a chart draws, so every bar keeps room for its label.
pub const MAX_CHART_ROWS: usize = 40;

/// The most series a chart draws, one theme colour each.
pub const MAX_CHART_SERIES: usize = 4;

/// `:::chart{type="..." title="..."}` with one header line and one line per
/// row, values split by commas, or `src="data.csv"` for a file beside the
/// post holding the same lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartBlock {
    /// How the numbers are drawn.
    pub kind: ChartKind,
    /// What the chart shows, its caption.
    pub title: String,
    /// What the numbers count, such as `%` or `visits`, shown beside them.
    pub unit: Option<String>,
    /// The CSV file beside the post the rows come from, when they are not
    /// written in the block.
    pub src: Option<String>,
    /// The numbers. Empty until the build reads `src`.
    pub table: ChartTable,
}

/// How a chart draws its numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    /// Bars across, for many rows or long labels.
    Bar,
    /// Columns upwards, for a few rows side by side.
    Column,
    /// A line through each series, for a change over time.
    Line,
}

impl ChartKind {
    /// The kind as written after `type=`, which is also its CSS class
    /// `chart-<name>`.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Bar => "bar",
            Self::Column => "column",
            Self::Line => "line",
        }
    }

    fn of(text: &str) -> Option<Self> {
        match text {
            "bar" => Some(Self::Bar),
            "column" => Some(Self::Column),
            "line" => Some(Self::Line),
            _ => None,
        }
    }
}

/// A chart's numbers: the header line, then one row per line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChartTable {
    /// The header of the labels' column, such as `Month`.
    pub label: String,
    /// The name of each series, the rest of the header.
    pub series: Vec<String>,
    /// The rows, in order.
    pub rows: Vec<ChartRow>,
}

/// One row of a chart: its label and one value per series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartRow {
    /// The row's label, such as `March`.
    pub label: String,
    /// One value per series.
    pub values: Vec<ChartValue>,
}

/// A number as written, such as `12.5`, checked to be one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartValue(String);

impl ChartValue {
    /// The number as written.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The number.
    #[must_use]
    pub fn number(&self) -> f64 {
        self.0.parse().unwrap_or_default()
    }

    fn try_new(text: &str) -> Option<Self> {
        let number: f64 = text.parse().ok()?;
        let written = text
            .bytes()
            .all(|b| b.is_ascii_digit() || b"-.".contains(&b));
        (written && number.is_finite()).then(|| Self(text.to_owned()))
    }
}

/// The body line a chart's lines go wrong on, and how to write them.
pub(crate) type Problem = (usize, &'static str);

/// The kind written as `type=`, a column chart when it is not, or `None`
/// for a kind there is not.
pub(crate) fn kind(text: Option<&str>) -> Option<ChartKind> {
    text.map_or(Some(ChartKind::Column), ChartKind::of)
}

/// Whether `src` names a CSV file right beside the post, such as
/// `traffic.csv`, so a chart never reads a file anywhere else.
pub(crate) fn is_file_name(src: &str) -> bool {
    let name = src.strip_suffix(".csv").unwrap_or_default();
    !name.is_empty()
        && !name.starts_with('.')
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
}

/// The table in `lines`: a header, then rows with as many cells. Blank
/// lines are skipped. `first` is the body line of `lines[0]`.
pub(crate) fn table(lines: &[&str], first: usize) -> Result<ChartTable, Problem> {
    let mut written = (first..)
        .zip(lines)
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(number, line)| (number, cells(line)));
    let Some((number, header)) = written.next() else {
        return Ok(ChartTable::default());
    };
    let Some((label, series)) = header.split_first() else {
        return Err((number, HEADER));
    };
    if series.is_empty() || series.len() > MAX_CHART_SERIES {
        return Err((number, HEADER));
    }
    let mut rows = Vec::new();
    for (number, cells) in written {
        let Some((row, values)) = cells.split_first() else {
            return Err((number, ROW));
        };
        if values.len() != series.len() {
            return Err((number, ROW));
        }
        let values = values
            .iter()
            .map(|value| ChartValue::try_new(value))
            .collect::<Option<Vec<_>>>()
            .ok_or((number, NUMBER))?;
        rows.push(ChartRow {
            label: row.clone(),
            values,
        });
    }
    if rows.is_empty() || rows.len() > MAX_CHART_ROWS {
        return Err((number, ROWS));
    }
    Ok(ChartTable {
        label: label.clone(),
        series: series.to_vec(),
        rows,
    })
}

const HEADER: &str = "a chart's first line names the labels' column, then one to four series, split by commas, such as Month, Visits";
const ROW: &str = "each line of a chart is a label, then one number per series in the first line, split by commas";
const NUMBER: &str = "a chart's values are plain numbers, such as 12 or -3.5, with no unit or thousands commas; put the unit in unit=";
const ROWS: &str = "a chart needs one to 40 rows under its first line";

/// The cells of one CSV line, trimmed. A cell in double quotes may hold
/// commas, and `""` inside it is one quote.
fn cells(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                cell.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => cells.push(std::mem::take(&mut cell).trim().to_owned()),
            _ => cell.push(c),
        }
    }
    cells.push(cell.trim().to_owned());
    cells
}

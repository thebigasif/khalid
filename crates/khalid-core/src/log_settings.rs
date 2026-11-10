//! What `khalid serve` writes to its log, from the `[log]` table of
//! `site.toml`.

use serde::Deserialize;

/// How much `khalid serve` says on standard error, and in which shape. Leave
/// the table out for plain lines at `info`:
///
/// ```toml
/// [log]
/// level = "warn"
/// format = "json"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct LogSettings {
    /// The least important line written, `info` unless set.
    #[serde(default)]
    pub level: LogLevel,
    /// Plain lines unless set, or one JSON object a line.
    #[serde(default)]
    pub format: LogFormat,
}

/// How important a log line is, least to most: a level writes its own lines
/// and every more important one. A release build leaves `debug` and `trace`
/// lines out when it is compiled, so there both log the same as `info`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum LogLevel {
    /// Every step, for finding a bug.
    Trace,
    /// Detail for a person looking into a problem.
    Debug,
    /// What the server did, such as a post going live.
    #[default]
    Info,
    /// Something failed and Khalid carried on, such as an email not sent.
    Warn,
    /// Something failed and a person must act.
    Error,
}

/// The shape of each log line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum LogFormat {
    /// One plain line a message, for a person reading the log.
    #[default]
    Text,
    /// One JSON object a line, for a log collector.
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_reads_and_left_out_is_plain_info() {
        let read: LogSettings = toml::from_str("level = \"warn\"\nformat = \"json\"").unwrap();
        assert_eq!(read.level, LogLevel::Warn);
        assert_eq!(read.format, LogFormat::Json);
        let empty: LogSettings = toml::from_str("").unwrap();
        assert_eq!(empty, LogSettings::default());
        assert_eq!(empty.level, LogLevel::Info);
        assert_eq!(empty.format, LogFormat::Text);
        assert!(LogLevel::Debug < LogLevel::Warn);
    }

    #[test]
    fn an_unknown_level_is_refused() {
        let refused = toml::from_str::<LogSettings>("level = \"loud\"");
        assert!(refused.is_err());
    }
}

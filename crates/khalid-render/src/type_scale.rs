//! The `[type]` and `[space]` tables of `theme/tokens.toml`: text sizes,
//! line height, heading weight and spacing (templates spec 4.3). The
//! defaults are the sizes the stylesheet had before they became tokens.

use std::cmp::Ordering;
use std::fmt;

use serde::Deserialize;

use crate::{Length, ParseTokenError};

/// The narrowest page a fluid size starts growing at, in rem (320 px).
const NARROW: f64 = 20.0;

/// The widest page a fluid size stops growing at, in rem (1280 px).
const WIDE: f64 = 80.0;

/// Text sizes, line height and heading weight.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TypeScale {
    /// Body text.
    pub body: FluidSize,
    /// The page title.
    pub h1: FluidSize,
    /// Section headings.
    pub h2: FluidSize,
    /// Subsection headings.
    pub h3: FluidSize,
    /// Fourth-level headings.
    pub h4: FluidSize,
    /// Dates, captions and other small text.
    pub small: FluidSize,
    /// The body's line height, such as `1.6`.
    pub line_height: LineHeight,
    /// How heavy headings are, such as `700`.
    pub heading_weight: Weight,
}

/// Spacing: the unit every gap is a multiple of, and the space between
/// the large parts of a page.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Space {
    /// The smallest gap, such as `0.5rem`.
    pub unit: Length,
    /// The space between sections, such as `2.5rem`.
    pub section: Length,
}

/// A text size: one rem size, or a smallest and a largest written
/// `"1rem 1.25rem"`, which grows with the page from 20rem to 80rem wide.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct FluidSize(String);

/// A line height from 1 to 2.5, such as `1.6`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(try_from = "f64")]
pub struct LineHeight(f64);

/// A font weight from 100 to 900 in steps of 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "u16")]
pub struct Weight(u16);

impl Default for TypeScale {
    fn default() -> Self {
        Self {
            body: FluidSize::trusted("clamp(1.0625rem, 1rem + 0.25vw, 1.1875rem)"),
            h1: FluidSize::trusted("clamp(1.875rem, 1.4rem + 2vw, 2.75rem)"),
            h2: FluidSize::trusted("clamp(1.4rem, 1.2rem + 1vw, 1.875rem)"),
            h3: FluidSize::trusted("clamp(1.15rem, 1.05rem + 0.5vw, 1.375rem)"),
            h4: FluidSize::trusted("1em"),
            small: FluidSize::trusted("0.9375rem"),
            line_height: LineHeight(1.6),
            heading_weight: Weight(700),
        }
    }
}

impl Default for Space {
    fn default() -> Self {
        Self {
            unit: Length::trusted("0.5rem"),
            section: Length::trusted("2.5rem"),
        }
    }
}

impl FluidSize {
    fn trusted(css: &str) -> Self {
        Self(css.to_owned())
    }

    /// The size as a CSS value, such as `clamp(1rem, 0.9167rem + 0.4167vw, 1.25rem)`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for FluidSize {
    type Error = ParseTokenError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        const WRONG: ParseTokenError = ParseTokenError(
            "a type size is one or two rem sizes, such as \"1.25rem\" or \"1rem 1.25rem\"",
        );
        let sizes: Vec<(&str, f64)> = text
            .split_whitespace()
            .map(|size| rem(size).map(|number| (size, number)).ok_or(WRONG))
            .collect::<Result<_, _>>()?;
        let [(least, small), (most, large)] = match sizes.as_slice() {
            [(one, _)] => return Ok(Self((*one).to_owned())),
            [first, second] => [*first, *second],
            _ => return Err(WRONG),
        };
        match small.partial_cmp(&large) {
            Some(Ordering::Less) => {
                let step = (large - small) / (WIDE - NARROW);
                let start = small - step * NARROW;
                let growth = step * 100.0;
                Ok(Self(format!(
                    "clamp({least}, {}rem + {}vw, {most})",
                    decimal(start),
                    decimal(growth)
                )))
            }
            Some(Ordering::Equal) => Ok(Self(least.to_owned())),
            _ => Err(ParseTokenError(
                "write the smaller size first, such as \"1rem 1.25rem\"",
            )),
        }
    }
}

/// The number of a rem size such as `1.25rem`, or `None`.
fn rem(size: &str) -> Option<f64> {
    let number = size.strip_suffix("rem")?;
    let plain = !number.is_empty() && number.chars().all(|c| c.is_ascii_digit() || c == '.');
    plain.then(|| number.parse().ok()).flatten()
}

/// `number` with at most four decimals and no trailing zeros, such as `0.9167`.
fn decimal(number: f64) -> String {
    let text = format!("{number:.4}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text == "-0" { "0" } else { text }.to_owned()
}

impl TryFrom<f64> for LineHeight {
    type Error = ParseTokenError;

    fn try_from(number: f64) -> Result<Self, Self::Error> {
        if (1.0..=2.5).contains(&number) {
            Ok(Self(number))
        } else {
            Err(ParseTokenError(
                "a line height is a number from 1 to 2.5, such as 1.6",
            ))
        }
    }
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u16> for Weight {
    type Error = ParseTokenError;

    fn try_from(number: u16) -> Result<Self, Self::Error> {
        if (100..=900).contains(&number) && number.is_multiple_of(100) {
            Ok(Self(number))
        } else {
            Err(ParseTokenError(
                "a weight is 100 to 900 in steps of 100, such as 700",
            ))
        }
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

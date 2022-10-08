use anyhow::{Error, Result};
use std::ops::RangeInclusive;

// parses something in the form of x=<a>..<b>
pub fn parse_raw_range(raw: &str) -> Result<RangeInclusive<isize>> {
    let mut bounds = raw.split('=');
    let _axis = bounds
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?;
    let mut values = bounds
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?
        .split("..");

    let lower_bound = values
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?
        .parse()?;
    let upper_bound = values
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?
        .parse()?;

    Ok(RangeInclusive::new(lower_bound, upper_bound))
}

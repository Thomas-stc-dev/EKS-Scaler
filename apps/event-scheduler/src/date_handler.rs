use chrono::{DateTime, Local, TimeZone, Datelike, Timelike};
use anyhow::{anyhow, Result};

pub fn parse_time(time_str: &str) -> Result<DateTime<Local>> {
    // Split the time string by ':'
    let parts: Vec<&str> = time_str.split(':').collect();
    
    if parts.len() != 2 {
        return Err(anyhow!("Invalid time format. Expected HH:MM"));
    }

    // Parse hours and minutes
    let hours: u32 = parts[0].parse()?;
    let minutes: u32 = parts[1].parse()?;

    // Validate hours and minutes
    if hours >= 24 || minutes >= 60 {
        return Err(anyhow!("Invalid time values"));
    }

    // Get today's date and set the time
    let now = Local::now();
    let datetime = Local.with_ymd_and_hms(
        now.year(),
        now.month(),
        now.day(),
        hours,
        minutes,
        0,
    ).single()
        .ok_or_else(|| anyhow!("Invalid date/time"))?;

    Ok(datetime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time() {
        let result = parse_time("09:00").unwrap();
        assert_eq!(result.hour(), 9);
        assert_eq!(result.minute(), 0);
    }

    #[test]
    fn test_invalid_time() {
        assert!(parse_time("25:00").is_err());
        assert!(parse_time("09:60").is_err());
        assert!(parse_time("invalid").is_err());
    }
}
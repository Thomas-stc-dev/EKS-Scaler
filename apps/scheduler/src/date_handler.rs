use anyhow::{anyhow, Result};

pub fn parse_time(time_str: &str) -> Result<[u32; 2]> {
    // Split the time string by ':'
    let parts: Vec<&str> = time_str.split(':').collect();
    
    if parts.len() != 2 {
        return Err(anyhow!("Invalid time format. Expected HH:MM"));
    }

    // Parse hours and minutes
    let hours: u32 = parts[0].parse()?;
    let minutes: u32 = parts[1].parse()?;

    Ok([hours, minutes])
}
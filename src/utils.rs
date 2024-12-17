use anyhow::Result;

pub fn parse_time_to_ms(time_str: &str) -> Result<f64> {
    let time_str = time_str.trim();

    if time_str.ends_with("us") {
        return time_str
            .trim_end_matches("us")
            .parse::<f64>()
            .map(|t| t / 1000.0)
            .map_err(|e| anyhow::anyhow!("Failed to parse time: {}", e));
    }

    if time_str.ends_with("ms") {
        return time_str
            .trim_end_matches("ms")
            .parse::<f64>()
            .map_err(|e| anyhow::anyhow!("Failed to parse time: {}", e));
    }

    if time_str.ends_with('s') && !time_str.ends_with("us") && !time_str.ends_with("ms") {
        return time_str
            .trim_end_matches('s')
            .parse::<f64>()
            .map(|t| t * 1000.0)
            .map_err(|e| anyhow::anyhow!("Failed to parse time: {}", e));
    }

    Err(anyhow::anyhow!("Invalid time format: {}", time_str))
}

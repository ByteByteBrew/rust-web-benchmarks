use anyhow::Result;

pub fn parse_time_to_ms(time_str: &str) -> Result<f64> {
    let parts: Vec<&str> = time_str.trim().split("us").collect();
    if parts.len() == 2 {
        parts[0]
            .trim()
            .parse::<f64>()
            .map(|t| t / 1000.0)
            .map_err(|e| anyhow::anyhow!("Failed to parse time: {}", e))
    } else {
        let parts: Vec<&str> = time_str.trim().split("ms").collect();
        if parts.len() == 2 {
            parts[0]
                .trim()
                .parse::<f64>()
                .map_err(|e| anyhow::anyhow!("Failed to parse time: {}", e))
        } else {
            Err(anyhow::anyhow!("Invalid time format: {}", time_str))
        }
    }
}

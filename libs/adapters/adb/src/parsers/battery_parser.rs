pub struct BatteryStatus {
    pub level: u32,
    pub is_charging: bool,
    pub temperature: f32,
}

pub struct BatteryParser;

impl BatteryParser {
    pub fn parse(output: &str) -> Option<BatteryStatus> {
        let mut level = None;
        let mut powered = false;
        let mut temp = 0.0;

        for line in output.lines() {
            let line = line.trim();
            if line.starts_with("level:") {
                level = line.split(':').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("AC powered:") || line.starts_with("USB powered:") {
                if line.contains("true") {
                    powered = true;
                }
            } else if line.starts_with("temperature:") {
                // Temp is usually in 0.1 degrees Celsius
                let raw_temp: f32 = line
                    .split(':')
                    .nth(1)
                    .and_then(|v| v.trim().parse().ok())
                    .unwrap_or(0.0);
                temp = raw_temp / 10.0;
            }
        }

        level.map(|l| BatteryStatus {
            level: l,
            is_charging: powered,
            temperature: temp,
        })
    }
}

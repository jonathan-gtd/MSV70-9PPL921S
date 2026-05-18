use std::path::Path;

#[derive(Debug, Clone)]
pub struct PidEntry {
    pub short_name:    String,
    pub pid_hex:       String,   // e.g. "0x5A81"
    pub pid_num:       u16,
    pub stat_name:     String,
    pub description_en: String,
    pub unit:          String,
    pub var_name:      String,
    pub data_type:     String,
    pub description_de: String,
}

impl PidEntry {
    pub fn matches(&self, query: &str) -> bool {
        if query.is_empty() { return true; }
        let q = query.to_lowercase();
        self.short_name.to_lowercase().contains(&q)
            || self.pid_hex.to_lowercase().contains(&q)
            || self.description_en.to_lowercase().contains(&q)
            || self.stat_name.to_lowercase().contains(&q)
            || self.unit.to_lowercase().contains(&q)
    }
}

pub fn load_pid_db(path: &Path) -> Result<Vec<PidEntry>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read PID DB: {e}"))?;

    let mut entries = Vec::new();
    for line in content.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 9 { continue; }

        let pid_str = cols[1].trim();
        let pid_num = u16::from_str_radix(pid_str.trim_start_matches("0x"), 16).unwrap_or(0);

        entries.push(PidEntry {
            short_name:     cols[0].trim().to_string(),
            pid_hex:        pid_str.to_string(),
            pid_num,
            stat_name:      cols[2].trim().to_string(),
            description_en: cols[3].trim().to_string(),
            unit:           cols[4].trim().to_string(),
            var_name:       cols[5].trim().to_string(),
            data_type:      cols[6].trim().to_string(),
            description_de: cols[8].trim().to_string(),
        });
    }

    Ok(entries)
}

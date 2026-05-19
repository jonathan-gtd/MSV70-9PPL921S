use egui::{Color32, FontId, RichText, ScrollArea, Ui};
use super::palette::*;

pub fn show(ui: &mut Ui, report: &str) {
    ScrollArea::both()
        .id_salt("report_scroll")
        .show(ui, |ui| {
            ui.style_mut().override_font_id = Some(FontId::monospace(10.5));
            for line in report.lines() {
                let color = classify_line(line);
                ui.label(RichText::new(line).color(color).font(FontId::monospace(10.5)));
            }
        });
}

fn classify_line(line: &str) -> Color32 {
    if line.contains('⛔') || line.contains('✗') || line.contains("CRITICAL") {
        RED
    } else if line.contains('⚠') || line.contains("WARN") || line.contains("LEAN") || line.contains("RICH") {
        ORANGE
    } else if line.contains('✓') || line.contains("[OK]") {
        GREEN
    } else if line.starts_with("  [") || line.contains('═') || line.contains("VERDICT") {
        ACCENT
    } else if line.contains('─') || line.trim().is_empty() {
        FG_DIM
    } else {
        FG
    }
}

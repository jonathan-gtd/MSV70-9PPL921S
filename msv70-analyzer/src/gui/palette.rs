use egui::Color32;

// ── UltraLog-exact background palette ────────────────────────────────────────
pub const BG:    Color32 = Color32::from_rgb(0x1a, 0x1a, 0x1a); // main bg
pub const BG2:   Color32 = Color32::from_rgb(0x25, 0x25, 0x26); // panel bg
pub const BG3:   Color32 = Color32::from_rgb(0x2d, 0x2d, 0x2d); // card bg
pub const BG4:   Color32 = Color32::from_rgb(0x3c, 0x3c, 0x3c); // active tab bg

// ── UltraLog brand colors ─────────────────────────────────────────────────────
pub const OLIVE:  Color32 = Color32::from_rgb(113, 120, 78);  // primary
pub const RUST:   Color32 = Color32::from_rgb(191, 78,  48);  // secondary / stop btn
pub const SAGE:   Color32 = Color32::from_rgb(159, 166, 119); // soft success
pub const AMBER:  Color32 = Color32::from_rgb(253, 193, 73);  // warning / playing
pub const DKBLUE: Color32 = Color32::from_rgb(71,  108, 155); // info
pub const CREAM:  Color32 = Color32::from_rgb(246, 247, 235); // main text
pub const CORN:   Color32 = Color32::from_rgb(100, 149, 237); // cornflower
pub const CORAL:  Color32 = Color32::from_rgb(255, 127, 80);  // coral
pub const LGREEN: Color32 = Color32::from_rgb(144, 238, 144); // light green / play btn
pub const DKRED:  Color32 = Color32::from_rgb(135, 30,  28);  // dark red

// ── Text ─────────────────────────────────────────────────────────────────────
pub const FG:     Color32 = CREAM;
pub const FG_DIM: Color32 = Color32::from_rgb(150, 150, 150);

// ── Aliases expected by existing code ────────────────────────────────────────
pub const ACCENT: Color32 = OLIVE;
pub const GREEN:  Color32 = LGREEN;
pub const YELLOW: Color32 = AMBER;
pub const ORANGE: Color32 = RUST;
pub const RED:    Color32 = Color32::from_rgb(0xf4, 0x47, 0x47); // bright alarm
pub const PURPLE: Color32 = Color32::from_rgb(0xc5, 0x86, 0xc0);

// ── UltraLog CHART_COLORS (same order as original) ───────────────────────────
pub const CHART: [Color32; 10] = [
    Color32::from_rgb(113, 120, 78),  // 0 olive
    Color32::from_rgb(191, 78,  48),  // 1 rust
    Color32::from_rgb(71,  108, 155), // 2 blue
    Color32::from_rgb(159, 166, 119), // 3 sage
    Color32::from_rgb(253, 193, 73),  // 4 amber
    Color32::from_rgb(135, 30,  28),  // 5 dark red
    Color32::from_rgb(246, 247, 235), // 6 cream
    Color32::from_rgb(100, 149, 237), // 7 cornflower
    Color32::from_rgb(255, 127, 80),  // 8 coral
    Color32::from_rgb(144, 238, 144), // 9 light green
];

// ── STFT coloring (by cfg thresholds, passed explicitly) ─────────────────────
pub fn stft_color(v: f64, note: f64, warn: f64, alarm: f64) -> Color32 {
    if v.abs() > alarm     { RED }
    else if v.abs() > warn  { RUST }
    else if v.abs() > note  { AMBER }
    else                    { SAGE }
}

pub fn phase_color(phase: &crate::data::Phase) -> Color32 {
    use crate::data::Phase;
    match phase {
        Phase::HotIdle  => LGREEN,
        Phase::Warmup   => CORN,
        Phase::Cruise   => OLIVE,
        Phase::Accel    => AMBER,
        Phase::Wot      => RUST,
        Phase::FuelCut  => FG_DIM,
        _               => FG_DIM,
    }
}

// ── Helper: draw an UltraLog-style filled button ──────────────────────────────
pub fn olive_btn(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let frame = egui::Frame::NONE
        .fill(OLIVE)
        .corner_radius(6.0)
        .inner_margin(egui::Margin::symmetric(14, 6));
    frame.show(ui, |ui| {
        ui.add(egui::Label::new(
            egui::RichText::new(text).color(Color32::WHITE).size(13.0)
        ).sense(egui::Sense::click()))
    }).inner
}

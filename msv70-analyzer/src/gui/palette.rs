use egui::Color32;

pub const BG:     Color32 = Color32::from_rgb(0x1e, 0x1e, 0x1e);
pub const BG2:    Color32 = Color32::from_rgb(0x25, 0x25, 0x26);
pub const BG3:    Color32 = Color32::from_rgb(0x2d, 0x2d, 0x30);
pub const ACCENT: Color32 = Color32::from_rgb(0x56, 0x9c, 0xd6);
pub const GREEN:  Color32 = Color32::from_rgb(0x4e, 0xc9, 0xb0);
pub const YELLOW: Color32 = Color32::from_rgb(0xdc, 0xdc, 0xaa);
pub const RED:    Color32 = Color32::from_rgb(0xf4, 0x47, 0x47);
pub const ORANGE: Color32 = Color32::from_rgb(0xce, 0x91, 0x78);
pub const FG:     Color32 = Color32::from_rgb(0xd4, 0xd4, 0xd4);
pub const FG_DIM: Color32 = Color32::from_rgb(0x6a, 0x6a, 0x6a);
pub const PURPLE: Color32 = Color32::from_rgb(0xc5, 0x86, 0xc0);

pub fn stft_color(v: f64) -> Color32 {
    use crate::analysis::Thresholds as THR;
    if v.abs() > THR::STFT_ALARM { RED }
    else if v.abs() > THR::STFT_WARN  { ORANGE }
    else if v.abs() > THR::STFT_NOTE  { YELLOW }
    else { GREEN }
}

pub fn phase_color(phase: &crate::data::Phase) -> Color32 {
    use crate::data::Phase;
    match phase {
        Phase::HotIdle  => GREEN,
        Phase::Warmup   => Color32::from_rgb(0x9c, 0xdc, 0xfe),
        Phase::Cruise   => ACCENT,
        Phase::Accel    => YELLOW,
        Phase::Wot      => RED,
        Phase::FuelCut  => FG_DIM,
        _               => FG_DIM,
    }
}

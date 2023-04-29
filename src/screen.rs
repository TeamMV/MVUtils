#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum Measurement {
    #[default]
    PX,
    MM,
    CM,
    DM,
    M,
    IN,
    FT
}

impl Measurement {
    pub fn compute(&self, dpi: f32, value: f32) -> f32 {
        match self {
            Measurement::PX => value,
            Measurement::MM => dpi / 25.4 * value,
            Measurement::CM => dpi / 2.54 * value,
            Measurement::DM => dpi / 0.254 * value,
            Measurement::M => dpi / 0.0254 * value,
            Measurement::IN => dpi * value,
            Measurement::FT => dpi * 12.0 * value
        }
    }
}
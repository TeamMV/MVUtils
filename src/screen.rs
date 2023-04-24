pub struct Measurements;
impl Measurements {
    pub fn compute(dpi: f32, value: f32, me: &Measurement) -> f32 {
        match me {
            Measurement::MM => { dpi / 25.4 * value}
            Measurement::CM => { dpi / 2.54 * value}
            Measurement::DM => { dpi / 0.254 * value}
            Measurement::M => { dpi / 0.0254 * value}
            Measurement::IN => { dpi * value}
            Measurement::FT => { dpi * value / 12.0}
        }
    }
}

pub enum Measurement {
    MM,
    CM,
    DM,
    M,
    IN,
    FT
}
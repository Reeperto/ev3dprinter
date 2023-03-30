use ev3dev_lang_rust::{motors::TachoMotor, Ev3Result};

pub struct Extruder {
    pub m: TachoMotor,
    pub ratio: f64,
}

impl Extruder {
    pub fn new(m: TachoMotor, ratio: f64) -> Self {
        Self { m, ratio }
    }

    pub fn mm_to_tacho(&self, measure: f64) -> Ev3Result<i32> {
        Ok((measure * self.ratio * (self.m.get_count_per_rot()? as f64 / 360.)).round() as i32)
    }
}

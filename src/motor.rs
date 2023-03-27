use ev3dev_lang_rust::{motors::LargeMotor, sensors::TouchSensor, Ev3Result};
use lazy_static::lazy_static;

lazy_static!(
    static ref ROOT_PATH: String = "/sys/class/".to_string();
);

pub struct Motor {
    pub m: LargeMotor,
    pub s: TouchSensor,
    pub ratio: f64,
    // Potentially replace with a struct or some other structure
    // (offset, motor inversion, sensor inversion)
    cal_params: (f64, bool, bool),
    cal_speed: i32,
}

impl Motor {
    pub fn new(
        m: LargeMotor,
        s: TouchSensor,
        ratio: f64,
        cal_params: (f64, bool, bool),
        cal_speed: i32,
    ) -> Self {
        Self {
            m,
            s,
            ratio,
            cal_params,
            cal_speed,
        }
    }


    pub fn calibrate(
        &self,
    ) -> Ev3Result<()> {
        let mut calibrate_speed = self.mm_to_tacho(self.cal_speed as f64)?;
        if self.cal_params.1 {
            calibrate_speed *= -1;
        }

        self.m.set_speed_sp(calibrate_speed)?;
        self.m.run_forever()?;
        self.wait_for_press(false ^ self.cal_params.2)?;

        self.m.set_speed_sp(-calibrate_speed)?;
        self.m.run_forever()?;
        self.wait_for_press(true ^ self.cal_params.2)?;

        self.m.stop()?;

        // Move to account for offset
        self.m.run_to_rel_pos(Some(self.mm_to_tacho(self.cal_params.0)?))?;

        Ok(())
    }

    pub fn mm_to_tacho(&self, measure: f64) -> Ev3Result<i32> {
        Ok((measure * self.ratio * (self.m.get_count_per_rot()? as f64 / 360.)).round() as i32)
    }

    pub fn wait_for_press(&self, inverted: bool) -> Ev3Result<()> {
        loop {
            if self.s.get_pressed_state()? ^ inverted {
                return Ok(());
            }
        }
    }

    // Fastest possible implentation of starting a motor to run forever
    // Bypasses abstractions and ev3dev library to achieve fastest possible timing
    pub fn quick_run(&self) {

    }
}

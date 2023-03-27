use std::{thread::sleep, time::Duration};

use cgmath::Vector2;
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

pub struct PrintHead {
    pub x: Motor,
    pub y: Motor,
    pub z: Motor,
    pub velocity: f64,
    pub position: Vector2<f64>
}

impl PrintHead {
    pub fn new(x: Motor, y: Motor, z: Motor) -> Self {
        Self {
            x,
            y,
            z,
            position: Vector2::new(0., 0.),
            // TODO:
            velocity: 20.
        }
    }

    pub fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }

    pub fn goto(&mut self, mut destination: Vector2<f64>) -> Ev3Result<()> {

        destination.x = destination.x.clamp(-100., 100.);
        destination.y = destination.y.clamp(-100., 100.);

        let delta_pos = destination - self.position;
        let time = f64::sqrt(delta_pos.x.powi(2) + delta_pos.y.powi(2)) / self.velocity;

        let velocity = delta_pos / time;

        let x_speed = self.x.mm_to_tacho(velocity.x)?;
        let y_speed = self.y.mm_to_tacho(velocity.y)?;

        self.x.m.set_speed_sp(x_speed)?;
        self.y.m.set_speed_sp(y_speed)?;

        self.x.m.run_forever()?;
        self.y.m.run_forever()?;

        sleep(Duration::from_secs_f64(time));

        self.x.m.stop()?;
        self.y.m.stop()?;

        self.position.x = destination.x;
        self.position.y = destination.y;

        Ok(())

    }

    pub fn calibrate(&self) -> Ev3Result<()> {
        self.x.m.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;
        self.y.m.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;

        self.x.calibrate()?;
        self.y.calibrate()?;
        // self.z.calibrate()?;

        Ok(())
    }
}

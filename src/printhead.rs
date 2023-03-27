use std::{thread::sleep, time::Duration};

use cgmath::Vector2;
use ev3dev_lang_rust::{motors::LargeMotor, Ev3Result};

use crate::motor::Motor;

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
            velocity: 20.
        }
    }

    pub fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }

    ///
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

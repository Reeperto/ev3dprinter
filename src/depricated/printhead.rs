use std::{thread::sleep, time::Duration};

use cgmath::Vector2;
use ev3dev_lang_rust::{motors::TachoMotor, Ev3Result};

use crate::motors::{velocity_to_tacho, PlaneMotor};

pub struct PrintHead {
    pub x_motor: PlaneMotor,
    pub y_motor: PlaneMotor,
    // e_motor: Whatever motor it is
    pub position: Vector2<f32>,
}

impl PrintHead {
    pub fn new(x_axis_motor: PlaneMotor, y_axis_motor: PlaneMotor) -> Self {
        Self {
            x_motor: x_axis_motor,
            y_motor: y_axis_motor,
            // z_motor: z_axis_motor,
            position: Vector2 { x: 0., y: 0. },
        }
    }

    pub fn goto(&mut self, mut position: Vector2<f32>, time: f32) -> Ev3Result<()> {
        position.x = position.x.clamp(-100., 100.);
        position.y = position.y.clamp(-100., 100.);

        let delta_pos = position - self.position;
        let velocity = delta_pos / time;

        let x_speed = velocity_to_tacho(
            velocity.x,
            self.x_motor.deg_mm_ratio,
            self.x_motor.tacho_per_rot,
        );

        let y_speed = velocity_to_tacho(
            velocity.y,
            self.y_motor.deg_mm_ratio,
            self.y_motor.tacho_per_rot,
        );

        self.x_motor.motor.set_speed_sp(x_speed)?;
        self.y_motor.motor.set_speed_sp(y_speed)?;

        // let tacho_pos_x =
        //     (position.x * self.x_motor.deg_mm_ratio * (self.x_motor.tacho_per_rot / 360.)) as i32;
        // let tacho_pos_y =
        //     (position.y * self.x_motor.deg_mm_ratio * (self.x_motor.tacho_per_rot / 360.)) as i32;
        //
        // self.x_motor.motor.run_to_abs_pos(Some(tacho_pos_x))?;
        // self.y_motor.motor.run_to_abs_pos(Some(tacho_pos_y))?;

        self.x_motor.motor.run_forever()?;
        self.y_motor.motor.run_forever()?;

        sleep(Duration::from_secs_f32(time));

        self.x_motor.motor.stop()?;
        self.y_motor.motor.stop()?;

        // TODO: Fix later
        self.position.x = position.x;
        self.position.y = position.y;

        sleep(Duration::from_millis(10));
        Ok(())
    }

    pub fn calibrate(
        &mut self,
        cal_x: bool,
        cal_y: bool,
        x_offset: f32,
        y_offset: f32,
    ) -> Ev3Result<()> {
        if cal_x {
            self.x_motor.calibrate(true, false, x_offset)?;
        }
        if cal_y {
            self.y_motor.calibrate(true, true, y_offset)?;
        }

        Ok(())
    }

    pub fn reset_position(&mut self) -> Ev3Result<()> {
        self.x_motor.motor.reset()?;
        self.y_motor.motor.reset()?;

        self.x_motor
            .motor
            .set_stop_action(TachoMotor::STOP_ACTION_HOLD)?;
        self.y_motor
            .motor
            .set_stop_action(TachoMotor::STOP_ACTION_HOLD)?;

        self.position = Vector2 { x: 0., y: 0. };

        Ok(())
    }
}

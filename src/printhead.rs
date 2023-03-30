use std::{thread::sleep, time::Duration};

use cgmath::Vector3;
use ev3dev_lang_rust::{motors::LargeMotor, Ev3Result};

use crate::{motor::Motor, extruder::Extruder};

// NOTE: Z motor does not work yet
#[allow(dead_code)]
pub struct PrintHead {
    x: Motor,
    y: Motor,
    z: Motor,
    e: Extruder,
    velocity: f64,
    position: Vector3<f64>,
    positioning: PositioningMode,
}

#[derive(Debug)]
pub enum PositioningMode {
    Relative,
    Absolute,
}

#[derive(Debug)]
pub struct LinearMoveCommand {
    pub point: Option<Vector3<f64>>,
    pub feed_rate: Option<f64>,
    pub extruded: Option<f64>,
}

impl PrintHead {
    pub fn new(x: Motor, y: Motor, z: Motor, e: Extruder) -> Self {
        Self {
            x,
            y,
            z,
            e,
            position: Vector3::new(0., 0., 0.),
            velocity: 20.,
            positioning: PositioningMode::Absolute,
        }
    }

    pub fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }

    pub fn set_position_mode(&mut self, mode: PositioningMode) {
        self.positioning = mode;
    }

    pub fn linear_move(&mut self, command: LinearMoveCommand) -> Ev3Result<()> {
        if let Some(mut destination) = command.point {

            if let Some(velocity) = command.feed_rate {
                self.velocity = velocity;
            }

            let delta_pos: Vector3<f64>;
            let time: f64;

            match self.positioning {
                PositioningMode::Absolute => {
                    // Is this the best idea?
                    destination.x = destination.x.clamp(-100., 100.);
                    destination.y = destination.y.clamp(-100., 100.);

                    delta_pos = destination - self.position;
                    time = f64::sqrt(delta_pos.x.powi(2) + delta_pos.y.powi(2)) / self.velocity;
                }
                PositioningMode::Relative => {
                    delta_pos = destination;
                    time = f64::sqrt(delta_pos.x.powi(2) + delta_pos.y.powi(2)) / self.velocity;
                }
            }

            let velocity = delta_pos / time;

            let x_speed = self.x.mm_to_tacho(velocity.x)?;
            let y_speed = self.y.mm_to_tacho(velocity.y)?;
            let z_speed = self.z.mm_to_tacho(velocity.z)?;

            self.x.m.set_speed_sp(x_speed)?;
            self.y.m.set_speed_sp(y_speed)?;
            self.z.m.set_speed_sp(z_speed)?;

            if let Some(extrude) = command.extruded {
                // Need speed of extrusion motor in tacho's counts per sec
                // Known info: time move takes in seconds, material to be extruded in mm.
                // 
                // Velocity of extrusion hence is extruded / time
                // Turn velocity into tacho with mm_to_tacho 
                self.e.m.set_speed_sp(self.e.mm_to_tacho(extrude / time)?)?;
                self.e.m.run_forever()?;
            }

            self.x.m.run_forever()?;
            self.y.m.run_forever()?;
            self.z.m.run_forever()?;

            sleep(Duration::from_secs_f64(time));

            self.e.m.stop()?;
            self.x.m.stop()?;
            self.y.m.stop()?;
            self.z.m.stop()?;

            self.position = destination;
        }

        Ok(())
    }

    pub fn calibrate(&self) -> Ev3Result<()> {
        self.x.m.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;
        self.y.m.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;
        self.z.m.set_stop_action(LargeMotor::STOP_ACTION_BRAKE)?;

        self.z.calibrate()?;
        self.x.calibrate()?;
        self.y.calibrate()?;

        Ok(())
    }
}

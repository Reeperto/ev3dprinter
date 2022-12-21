mod motors;

extern crate ev3dev_lang_rust;

use std::thread::sleep;
use std::time::Duration;
use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::MotorPort;
use ev3dev_lang_rust::sensors::SensorPort;
use crate::motors::AxisMotor;

fn main() -> Ev3Result<()> {

    let mut x_motor = AxisMotor::new(MotorPort::OutB, SensorPort::In2, 20f32, 5f32);
    let mut y_motor = AxisMotor::new(MotorPort::OutA, SensorPort::In1, 20f32, 14.0625f32);

    y_motor.invert(true)?;

    // Calibrate Motors
    x_motor.calibrate()?;
    y_motor.calibrate()?;

    sleep (Duration::new(2,0));

    x_motor.move_pos (-20.4)?;
    y_motor.move_pos (-20.4)?;

    Ok(())
}

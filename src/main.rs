use std::{thread::sleep, time::Duration};

use cgmath::Vector2;
use ev3dev_lang_rust::{
    motors::{MotorPort, TachoMotor},
    sensors::{SensorPort, TouchSensor},
    Ev3Result,
};
use printhead::{Motor, PrintHead, SensorPool};

mod printhead;

extern crate ev3dev_lang_rust;

fn main() -> Ev3Result<()> {
    let x_motor: Motor = Motor {
        m: TachoMotor::get(MotorPort::OutA)?,
        deg_mm_ratio: 5.,
    };

    let y_motor: Motor = Motor {
        m: TachoMotor::get(MotorPort::OutB)?,
        deg_mm_ratio: 14.0625,
    };

    let z_motor = Motor {
        m: TachoMotor::get(MotorPort::OutC)?,
        deg_mm_ratio: 240.,
    };

    let sensor_pool: SensorPool = SensorPool {
        x: TouchSensor::get(SensorPort::In1)?,
        y: TouchSensor::get(SensorPort::In2)?,
        z: TouchSensor::get(SensorPort::In3)?,
    };

    let mut printhead: PrintHead = PrintHead::new(x_motor, y_motor, z_motor, sensor_pool, 15., 1.);

    printhead.calibrate_head(true, true, false, 0., -20.4, 0.)?;
    sleep(Duration::new(1, 0));
    printhead.reset_position()?;

    printhead.goto(Vector2 { x: 20., y: 20. }, 1.)?;
    printhead.goto(Vector2 { x: 20., y: 40. }, 1.)?;
    printhead.goto(Vector2 { x: 40., y: 40. }, 1.)?;
    printhead.goto(Vector2 { x: 40., y: 20. }, 1.)?;
    printhead.goto(Vector2 { x: 20., y: 20. }, 1.)?;

    Ok(())
}

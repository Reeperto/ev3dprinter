use ev3dev_lang_rust::{
    motors::{MotorPort, TachoMotor},
    sensors::{SensorPort, TouchSensor},
    Ev3Result,
};
use ev3dlib::{printhead::PrintHead, motor::Motor, gcode::parser::{run_gcode, parse_gcode_file}, extruder::Extruder};

fn main() -> Ev3Result<()> {
    let x_motor = Motor::new(
        TachoMotor::get(MotorPort::OutA)?,
        TouchSensor::get(SensorPort::In1)?,
        5.,
        (0., true, false),
        10,
    );
    let y_motor = Motor::new(
        TachoMotor::get(MotorPort::OutB)?,
        TouchSensor::get(SensorPort::In2)?,
        14.0625,
        (-20.4, true, true),
        10,
    );
    let z_motor = Motor::new(
        TachoMotor::get(MotorPort::OutC)?,
        TouchSensor::get(SensorPort::In3)?,
        240.,
        (0., true, false),
        1,
    );
    let e_motor = Extruder::new(
        TachoMotor::get(MotorPort::OutD)?,
        // 78.5398163397 per 1 rot of wheel
        // Motor rotates 8 times for 1 rotation of wheel
        // 9.8174770425 mm per rot of motor
        9.8174770425
    );

    e_motor.m.set_polarity(TachoMotor::POLARITY_INVERSED)?;

    let mut head = PrintHead::new(
        x_motor,
        y_motor,
        z_motor,
        e_motor
    );

    head.calibrate()?;
    let instrs = parse_gcode_file("./example.gcode".to_string()).unwrap();
    run_gcode(&mut head, instrs)?;

    Ok(())
}

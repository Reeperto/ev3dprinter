use ev3dev_lang_rust::{
    motors::{MotorPort, LargeMotor},
    sensors::{SensorPort, TouchSensor},
    Ev3Result,
};
use ev3dlib::{printhead::{Motor, PrintHead}, gcode::{parse_gcode_file, run_gcode}};

fn main() -> Ev3Result<()> {
    let x_motor = Motor::new(
        LargeMotor::get(MotorPort::OutA)?,
        TouchSensor::get(SensorPort::In1)?,
        5.,
        (0., true, false),
        10,
    );
    let y_motor = Motor::new(
        LargeMotor::get(MotorPort::OutB)?,
        TouchSensor::get(SensorPort::In2)?,
        14.0625,
        (-20.4, true, true),
        10,
    );
    let z_motor = Motor::new(
        LargeMotor::get(MotorPort::OutC)?,
        TouchSensor::get(SensorPort::In3)?,
        240.,
        (0., false, false),
        10,
    );

    let mut head = PrintHead::new(
        x_motor,
        y_motor,
        z_motor,
    );

    head.calibrate()?;
    let instrs = parse_gcode_file("./example.gcode".to_string()).unwrap();
    run_gcode(&mut head, instrs)?;

    Ok(())
}

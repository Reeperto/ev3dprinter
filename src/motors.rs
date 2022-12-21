use ev3dev_lang_rust::{Ev3Result};
use ev3dev_lang_rust::motors::{MotorPort, TachoMotor};
use ev3dev_lang_rust::sensors::{SensorPort, TouchSensor};

#[derive(Debug)]
pub struct AxisMotor {
    motor: TachoMotor,
    sensor: TouchSensor,
    base_speed: i32,
    inverted: bool,
    deg_ratio: f32,
    tacho_ratio: f32,
    position: f32
}

impl AxisMotor {
    /// Returns an encapsulated Axis Motor with the given ports and defaults
    ///
    /// # Arguments
    /// * `m_port` - The EV3 port that the motor is connected to
    /// * `s_port` - The EV3 port that the associated touch sensor is connected to
    /// * `base_velocity` - The default speed the axis motor travels at in millimeters/second
    /// * `mm_ratio` - The ratio of degrees a motor turns per millimeter of movement
    ///
    pub fn new(m_port: MotorPort, s_port: SensorPort, base_velocity: f32, mm_ratio: f32) -> Self {
        let _m = TachoMotor::get(m_port).expect("Motor not found at port");
        let _tacho_ratio: f32 = _m.get_count_per_rot().expect("Unable to get motor tacho count ratio") as f32;
        let _speed: i32 = (base_velocity * mm_ratio * (_tacho_ratio / 360f32)).round() as i32;

        _m.set_stop_action(TachoMotor::STOP_ACTION_HOLD).expect("Unable to change motor's stop action");

        Self {
            motor: _m,
            sensor: TouchSensor::get(s_port).expect("Touch Sensor not found at port"),
            base_speed: _speed,
            inverted: false,
            deg_ratio: mm_ratio,
            tacho_ratio: _tacho_ratio,
            position: 0f32
        }
    }

    pub fn invert(&mut self, invert: bool) -> Ev3Result<()> {

        if !invert {
            self.inverted = invert;
            self.motor.set_polarity (TachoMotor::POLARITY_NORMAL)?;
            return Ok(());
        }

        self.inverted = invert;
        self.motor.set_polarity (TachoMotor::POLARITY_INVERSED)?;

        Ok(())
    }

    /* TODO: Currently both motors do not move the plate in the right direction.
        A negative value should move the y axis down, but it instead moves up. The x axis also
        moves to left with a positive value.
        *** Overall, find a better method for handling inverting motors ****/
    pub fn move_pos(&self, coordinate: f32) -> Ev3Result<()> {

        let delta_deg = (coordinate - self.position) * self.deg_ratio;
        let mut del_pos = (delta_deg * self.tacho_ratio / 360.).round() as i32;

        self.motor.set_speed_sp (self.base_speed)?;
        self.motor.run_to_rel_pos (Some(del_pos))?;

        Ok(())
    }

    pub fn calibrate(&self) -> Ev3Result<()> {

        self.motor.set_speed_sp (self.base_speed)?;
        self.motor.run_forever ()?;

        wait_for_press (&self.sensor, true)?;

        self.motor.set_speed_sp (-self.base_speed / 2)?;
        self.motor.run_forever ()?;

        wait_for_press (&self.sensor,false)?;

        self.motor.stop()?;

        Ok(())
    }

}

fn wait_for_press(sensor: &TouchSensor, invert: bool) -> Ev3Result<()> {
    loop {
        let pressed = sensor.get_pressed_state()? ^ invert;
        if pressed {
            return Ok(());
        }
    }
}

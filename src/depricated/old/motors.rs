use ev3dev_lang_rust::motors::{MotorPort, TachoMotor};
use ev3dev_lang_rust::sensors::{SensorPort, TouchSensor};
use ev3dev_lang_rust::Ev3Result;

pub struct PlaneMotor {
    pub motor: TachoMotor,
    pub sensor: TouchSensor,
    pub base_speed: i32,
    pub deg_mm_ratio: f32,
    pub tacho_per_rot: f32,
}

pub struct LayerMotor {
    pub motor: PlaneMotor,
    pub layer_height: f32,
    pub current_layer: i32,
}

impl LayerMotor {
    pub fn new(motor: PlaneMotor, layer_height: f32) -> Self {
        Self {
            motor,
            layer_height,
            current_layer: 0,
        }
    }
}

impl PlaneMotor {
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

        _m.reset().expect("Unable to reset attached motor");

        let _tacho_per_rot: f32 =
            _m.get_count_per_rot()
                .expect("Unable to get motor tacho count ratio") as f32;

        let _speed: i32 = velocity_to_tacho(base_velocity, mm_ratio, _tacho_per_rot);

        _m.set_stop_action(TachoMotor::STOP_ACTION_HOLD)
            .expect("Unable to change motor's stop action");

        Self {
            motor: _m,
            sensor: TouchSensor::get(s_port).expect("Touch Sensor not found at port"),
            base_speed: _speed,
            deg_mm_ratio: mm_ratio,
            tacho_per_rot: _tacho_per_rot,
        }
    }

    pub fn delta_goto(&self, position: f32, velocity: f32) -> Ev3Result<()> {
        let deg = position * self.deg_mm_ratio;
        // Converting degrees to tacho counts -> deg * tacho/rot * 1 rot/360 degree
        let tacho = (deg * self.tacho_per_rot / 360.).round() as i32;

        self.motor.set_speed_sp(velocity_to_tacho(
            velocity,
            self.deg_mm_ratio,
            self.tacho_per_rot,
        ))?;

        self.motor.run_to_rel_pos(Some(tacho))?;

        self.motor.wait_until_not_moving(None);

        Ok(())
    }

    pub fn calibrate(
        &mut self,
        inverted_motor: bool,
        inverted_sensor: bool,
        offset: f32,
    ) -> Ev3Result<()> {
        let mut calibrate_speed = self.base_speed;
        if inverted_motor {
            calibrate_speed = calibrate_speed * -1;
        }

        self.motor.set_speed_sp(calibrate_speed)?;
        self.motor.run_forever()?;

        wait_for_press(&self.sensor, false ^ inverted_sensor)?;

        self.motor.set_speed_sp(-calibrate_speed)?;
        self.motor.run_forever()?;

        wait_for_press(&self.sensor, true ^ inverted_sensor)?;

        self.motor.stop()?;

        self.delta_goto(offset, 15.)?;

        Ok(())
    }
}

pub fn velocity_to_tacho(velocity: f32, deg_mm_ratio: f32, tacho_ratio: f32) -> i32 {
    (velocity * deg_mm_ratio * (tacho_ratio / 360f32)).round() as i32
}

fn wait_for_press(sensor: &TouchSensor, invert: bool) -> Ev3Result<()> {
    loop {
        let pressed = sensor
            .get_pressed_state()
            .expect("Unable to get touchsensor state")
            ^ invert;
        if pressed {
            return Ok(());
        }
    }
}

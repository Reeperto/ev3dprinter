use std::{thread::sleep, time::Duration};

use cgmath::Vector2;
use ev3dev_lang_rust::{motors::TachoMotor, sensors::TouchSensor, Ev3Result};

pub struct Motor {
    pub m: TachoMotor,
    pub deg_mm_ratio: f32,
}

pub struct SensorPool {
    pub x: TouchSensor,
    pub y: TouchSensor,
    pub z: TouchSensor,
}

pub struct PrintHead {
    pub x: Motor,
    pub y: Motor,
    pub z: Motor,
    pub sensors: SensorPool,
    pub base_velocity: f32,
    pub position: Vector2<f32>,
    pub layer: i32,
    pub layer_height: f32,
}

impl PrintHead {
    pub fn new(
        x: Motor,
        y: Motor,
        z: Motor,
        sensors: SensorPool,
        base_velocity: f32,
        layer_height: f32,
    ) -> Self {
        Self {
            x,
            y,
            z,
            sensors,
            base_velocity,
            position: Vector2 { x: 0., y: 0. },
            layer: 0,
            layer_height,
        }
    }

    pub fn goto(&mut self, mut position: Vector2<f32>, time: f32) -> Ev3Result<()> {
        position.x = position.x.clamp(-100., 100.);
        position.y = position.y.clamp(-100., 100.);

        let delta_pos: Vector2<f32> = position - self.position;
        let velocity: Vector2<f32> = delta_pos / time;

        let x_speed: i32 = self.mm_to_tacho(&self.x, velocity.x);
        let y_speed: i32 = self.mm_to_tacho(&self.y, velocity.y);

        self.x.m.set_speed_sp(x_speed)?;
        self.y.m.set_speed_sp(y_speed)?;

        // XXX: Implement a kernel device level approach to starting these together
        // ------------------------------------------

        self.x.m.run_forever()?;
        self.y.m.run_forever()?;

        sleep(Duration::from_secs_f32(time));

        self.x.m.stop()?;
        self.y.m.stop()?;
        //XXX: ------------------------------------------

        // TODO: Implement more intelligent position logging with the absolute tacho count of the
        // motors
        self.position.x = position.x;
        self.position.y = position.y;

        // Prevents drifting between succesive goto calls
        sleep(Duration::from_millis(10));
        Ok(())
    }

    pub fn calibrate_head(
        &self,
        x_cal: bool,
        y_cal: bool,
        z_cal: bool,
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
    ) -> Ev3Result<()> {
        if x_cal {
            self._calibrate_motor(&self.x, &self.sensors.x, true, false, x_offset)?;
        }
        if y_cal {
            self._calibrate_motor(&self.y, &self.sensors.y, true, true, y_offset)?;
        }
        if z_cal {
            self._calibrate_motor(&self.z, &self.sensors.z, false, false, z_offset)?;
        }
        Ok(())
    }

    fn _calibrate_motor(
        &self,
        motor: &Motor,
        sensor: &TouchSensor,
        inverted_motor: bool,
        inverted_sensor: bool,
        offset: f32,
    ) -> Ev3Result<()> {
        let mut calibrate_speed = self.mm_to_tacho(&motor, self.base_velocity);
        if inverted_motor {
            calibrate_speed = calibrate_speed * -1;
        }

        motor.m.set_speed_sp(calibrate_speed)?;
        motor.m.run_forever()?;

        self.wait_for_press(sensor, false ^ inverted_sensor)?;

        motor.m.set_speed_sp(-calibrate_speed)?;
        motor.m.run_forever()?;

        self.wait_for_press(sensor, true ^ inverted_sensor)?;

        motor.m.stop()?;

        motor
            .m
            .run_to_rel_pos(Some(self.mm_to_tacho(motor, offset)))?;

        Ok(())
    }

    pub fn reset_position(&mut self) -> Ev3Result<()> {
        self.x.m.reset()?;
        self.y.m.reset()?;

        self.x.m.set_stop_action(TachoMotor::STOP_ACTION_HOLD)?;
        self.y.m.set_stop_action(TachoMotor::STOP_ACTION_HOLD)?;

        self.position = Vector2 { x: 0., y: 0. };

        Ok(())
    }

    fn wait_for_press(&self, sensor: &TouchSensor, invert: bool) -> Ev3Result<()> {
        loop {
            let pressed = sensor
                .get_pressed_state()
                .expect("[ERROR]: Unable to get touchsensor state")
                ^ invert;
            if pressed {
                return Ok(());
            }
        }
    }

    fn mm_to_tacho(&self, motor: &Motor, metric: f32) -> i32 {
        (metric
            * motor.deg_mm_ratio
            * (motor
                .m
                .get_count_per_rot()
                .expect("[ERROR]: Cannot get tacho counts per rotation") as f32
                / 360.))
            .round() as i32
    }
}

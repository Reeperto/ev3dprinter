package ev3dprinter.motors;

import ev3dprinter.devices.BaseMotor;
import lejos.hardware.port.Port;
import lejos.utility.Delay;

// TODO Implement a method that converts a millimeter distance along the spool to a degree angle to turn.
// TODO Implement a method to move the print head to a given millimeter height.

public class ZMotor extends BaseMotor {
    public ZMotor(Port port, Port sensorAddress, float defaultSpeed, float degMmRatio, Boolean ev3TouchSensor) {
        super(port, sensorAddress, defaultSpeed, degMmRatio, ev3TouchSensor);
    }

}

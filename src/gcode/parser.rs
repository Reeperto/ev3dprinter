use std::{
    fs::File,
    io::{self, BufRead},
    thread::sleep,
    time::Duration,
};

use cgmath::Vector3;
use easy_error::{Result, ResultExt};
use ev3dev_lang_rust::Ev3Result;

use crate::printhead::{LinearMoveCommand, PositioningMode, PrintHead};

#[derive(Debug)]
pub enum Instruction {
    LinearMove(LinearMoveCommand),
    Pause(f64),
    SetPositioning(PositioningMode),
    Release(bool, bool, bool),
}

pub fn parse_gcode_file(filename: String) -> Result<Vec<Instruction>> {
    let file = File::open(filename).context("Unable to read file")?;
    let mut instructions: Vec<Instruction> = vec![];

    for line in io::BufReader::new(file).lines() {
        let line = line.context("Cannot read line of file")?;
        let command: Vec<&str> = line.split_whitespace().collect();

        if let Some(com) = command.first() {
            match *com {
                // Interpreting both a fast move and normal move the same
                "G0" | "G1" => {
                    let mut pos = Vector3::new(0., 0., 0.);
                    let mut feed_rate = 0.;
                    let mut extruded = 0.;

                    // Gather all parameters
                    for param in command.iter().skip(1) {
                        let iden = param.chars().next().unwrap();
                        match iden {
                            'X' => {
                                let mut chars = param.chars();
                                chars.next();
                                pos.x = chars.as_str().parse::<f64>().unwrap()
                            }
                            'Y' => {
                                let mut chars = param.chars();
                                chars.next();
                                pos.y = chars.as_str().parse::<f64>().unwrap()
                            }
                            'Z' => {
                                let mut chars = param.chars();
                                chars.next();
                                pos.z = chars.as_str().parse::<f64>().unwrap()
                            }
                            'E' => {
                                let mut chars = param.chars();
                                chars.next();
                                extruded = chars.as_str().parse::<f64>().unwrap()
                            }
                            'F' => {
                                let mut chars = param.chars();
                                chars.next(); 
                                feed_rate = chars.as_str().parse::<f64>().unwrap() / 60.
                            }
                            ';' => {
                                break;
                            }
                            _ => {}
                        }
                    }

                    let com = LinearMoveCommand {
                        point: if pos != Vector3::new(0., 0., 0.) {
                            Some(pos)
                        } else {
                            None
                        },
                        feed_rate: if feed_rate != 0. {
                            Some(feed_rate)
                        } else {
                            None
                        },
                        extruded: if extruded != 0. { Some(extruded) } else { None },
                    };

                    instructions.push(Instruction::LinearMove(com));
                }
                "G4" => instructions.push(Instruction::SetPositioning(PositioningMode::Absolute)),
                "G90" => instructions.push(Instruction::SetPositioning(PositioningMode::Absolute)),
                "G91" => instructions.push(Instruction::SetPositioning(PositioningMode::Relative)),
                "M18" => todo!("Properly parse release command"),
                _ => {}
            }
        }
    }

    Ok(instructions)
}

pub fn run_gcode(printhead: &mut PrintHead, instructions: Vec<Instruction>) -> Ev3Result<()> {
    for instr in instructions {
        match instr {
            Instruction::LinearMove(com) => {
                printhead.linear_move(com)?;
            }
            Instruction::SetPositioning(mode) => {
                printhead.set_position_mode(mode);
            }
            Instruction::Pause(delay) => {
                sleep(Duration::from_secs_f64(delay));
            }
            Instruction::Release(_rx, _ry, _rz) => {
                todo!("Properly release all motors (set brake behavior to be none)")
            }
        }
    }

    Ok(())
}

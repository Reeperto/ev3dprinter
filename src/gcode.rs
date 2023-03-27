use std::{fs::File, io::{self, BufRead}};

use cgmath::Vector2;
use easy_error::{Result, ResultExt};
use ev3dev_lang_rust::Ev3Result;

use crate::printhead::PrintHead;


#[derive(Debug)]
pub enum Instruction {
    Goto(Vector2<f64>, f64)
}

pub fn parse_gcode_file(filename: String) -> Result<Vec<Instruction>> {

    let file = File::open(filename).context("Unable to read file")?;
    let mut instructions: Vec<Instruction> = vec![];

    for line in io::BufReader::new(file).lines() {

        let line = line.context("Cannot read line of file")?;
        let command: Vec<&str> = line.split_whitespace().collect();

        if let Some(com) = command.get(0) {
            match *com {
                "G1" => {
                    let mut pos = Vector2::new(0f64, 0f64);
                    for param in command.iter().skip(1) {
                        let iden = param.chars().next().unwrap();
                        match iden {
                            'X' => {
                                let mut chars = param.chars();
                                chars.next();
                                pos.x = chars.as_str().parse::<f64>().unwrap()
                            },
                            'Y' => {
                                let mut chars = param.chars();
                                chars.next();
                                pos.y = chars.as_str().parse::<f64>().unwrap()
                            },
                            ';' => {
                                break;
                            },
                            _ => {}
                        }
                    }
                    instructions.push(Instruction::Goto(pos, 1.))
                }
                _ => {}
            }

        }
    }

    Ok(instructions)
}

pub fn run_gcode(printhead: &mut PrintHead, instructions: Vec<Instruction>) -> Ev3Result<()> {

    for instr in instructions {
        match instr {
            Instruction::Goto(pos, _) => {
                printhead.goto(pos)?;
            }
        }
    }

    Ok(())

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gcode_file() {
        let filename = "/Users/reeperto/dev/git/ev3dprinter/example.gcode".to_string();
        let instrs = parse_gcode_file(filename);
        println!("{:#?}", instrs);
    }
}

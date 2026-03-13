use env_logger::Env;
use log::{info, trace};
use std::env;
use std::fs;

use aoc_lib::AocError;

/// Advent of Code 2025 Day 1
/// <https://adventofcode.com/2025/day/1>
///
/// We're given a list of dial turns. Like L20 to turn it left 20 noches, or R4
/// to turn it right 4 notches. The outside of the dial is numbered from 0-99,
/// and starts pointing at 50.
const DIAL_SIZE: u32 = 100;

struct Dial {
    /// The dial has a position from 0-99
    pos: u32,
    /// The number of times the dial stopped at 0
    zero_stops: u32,
    /// The number of times the dial crossed but didn't stop at 0
    zeros_crossed: u32,
}

/// The dial starts at 50
impl Default for Dial {
    fn default() -> Self {
        Self {
            pos: 50,
            zero_stops: 0,
            zeros_crossed: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    /// A left-rotation by N notches
    Left(u32),
    /// A right-rotation by N notches
    Right(u32),
}

impl Rotation {
    /// Parse an instruction like 'L68' or 'R48' into an instruciton like `Rotation::Left(86)` or `Rotation::Right(48)`
    fn parse(rot_str: &str) -> anyhow::Result<Self> {
        // Split at index 1 to separatate the direction (L or R) from the number
        // of notches to turn the dial by
        let (dir_str, amount_str) = rot_str.split_at(1);

        let amount = amount_str.parse::<u32>()?;

        match dir_str {
            "L" => Ok(Self::Left(amount)),
            "R" => Ok(Self::Right(amount)),
            _ => Err(AocError::ParseError(dir_str.into()).into()),
        }
    }
}

impl Dial {
    /// Rotate the dial, noting zero-stops and zero-crossings
    fn rotate(&mut self, rot: &Rotation) {
        let start_pos = self.pos;

        // Do some modular arithmetic to find what notch the dial stops at,
        // and how many times it saw zero

        let new_position_premod = match rot {
            Rotation::Left(dist) => {
                trace!("Rotating left by {:?} from {:?}", dist, self.pos);
                start_pos as i32 - *dist as i32
            }
            Rotation::Right(dist) => {
                trace!("Rotating right by {:?} from {:?}", dist, self.pos);
                start_pos as i32 + *dist as i32
            }
        };

        // Update our position and number of zeros crossed
        self.pos = new_position_premod.rem_euclid(DIAL_SIZE as i32) as u32;
        let mut zeros_crossed = new_position_premod
            .div_euclid(DIAL_SIZE as i32)
            .unsigned_abs();

        match rot {
            Rotation::Left(_dist) => {
                // If we started at 0 and rotated left, don't count that
                // zero-crossing since it would have been counted as a 'stop a
                // zero' during the last rotation
                if start_pos == 0 {
                    zeros_crossed -= 1;
                }
            }
            Rotation::Right(_dist) => {
                // If we're at zero now, it means we rotated right through to
                // the 0. Subtract that zero-crossing since it'll get counted as
                // a 'stop at 0'
                if self.pos == 0 {
                    zeros_crossed -= 1;
                }
            }
        }

        if self.pos == 0 {
            self.zero_stops += 1
        }

        self.zeros_crossed += zeros_crossed;

        trace!(
            " New pos: {:?} new_pos_remod: {:?} Zero crossings: {:?}",
            self.pos, new_position_premod, zeros_crossed
        );
    }

    /// Apply rotations one by one
    fn apply_rotations(&mut self, rotations: &[Rotation]) {
        for rotation in rotations {
            self.rotate(rotation);
        }
    }
}

fn parse_rotations(rotations_str: &str) -> anyhow::Result<Vec<Rotation>> {
    rotations_str.lines().map(Rotation::parse).collect()
}

fn parse_input_file(input_filename: &str) -> anyhow::Result<Vec<Rotation>> {
    let input_str = fs::read_to_string(input_filename).expect("Able to read input file");
    parse_rotations(&input_str)
}

fn create_dial_and_apply_input_file_to_it(input_filename: &str) -> Dial {
    let rotations = parse_input_file(input_filename).expect("Able to parse input file");

    let mut dial = Dial::default();
    dial.apply_rotations(&rotations);

    dial
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_filename = format!("{manifest_dir}/src/input.txt");

    let dial = create_dial_and_apply_input_file_to_it(&input_filename);

    info!("Zeros stopped at: {:?}", dial.zero_stops);
    info!("Additional zero crossings: {:?}", dial.zeros_crossed);
    info!(
        "Total zeros visited: {:?}",
        dial.zero_stops + dial.zeros_crossed
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rotations() {
        let str_to_parse = "L10\nR20";

        let expected = vec![Rotation::Left(10), Rotation::Right(20)];
        let actual = parse_rotations(str_to_parse).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sample_input() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let input_filename = format!("{manifest_dir}/src/test_input.txt");
        let dial = create_dial_and_apply_input_file_to_it(&input_filename);

        // Dial should stop at 3 times
        assert_eq!(3, dial.zero_stops);

        // Dial should see 0 6 times
        assert_eq!(6, dial.zero_stops + dial.zeros_crossed);
    }
}

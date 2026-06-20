use aoc_lib::{AocError, load_input_file_in_src_dir_to_string};
use env_logger::Env;
use log::info;

/// Advent of Code 2025 Day 3
/// <https://adventofcode.com/2025/day/3>
///
/// We're given battery banks where each cell has a joltage from 0-9.
/// We need to find out the maximum joltage we can get out of the battery.
///
/// Part 1: You can activate two cells. The 2-digits formed by the left-right
/// ordering of the contituent cells' joltages gives the overall joltage.
///
/// Part 2: Like part 1, but you have to active 12 cells instead of 2.
///
/// The problem calls this a Bank but I think a better naming is to call it a
/// `Battery`, and to call the constituent cells in it Cells instead of
/// batteries. A battery is multiple cells piled up.
///
/// I did this by iterating through the rightmost cells, and finding the cell
/// furthest to the left of it that isn't activated yet.
type Battery = Vec<u8>;

/// Get the digit at position 'n' of a `Battery` where 0 is least significant digit
fn get_digit_at_n(bank: &Battery, n: usize) -> u64 {
    let index = bank.len() - n - 1;
    bank[index] as u64
}

/// Given a `Vec` of cells that are on, calculate the joltage
fn calc_joltage_from_on_cells(bank: &Battery, on_switches: Vec<usize>) -> u64 {
    let num_on = on_switches.len();

    // iterate through the cells that are on, multiply it by an appropriate value of
    // 10 depending on its place value, and add these all up
    on_switches
        .into_iter()
        .enumerate()
        .map(|(i, on_switch_idx)| {
            let value_at_index = get_digit_at_n(bank, on_switch_idx);
            let exp = num_on - i - 1;
            value_at_index * 10u64.pow(exp as u32)
        })
        .sum()
}

/// Helper for `calc_biggest_joltage_possible_for_battery()`.  Given an index
/// into a `Battery`'s cells, and the index of the leftmost cell that's already
/// activated, what's the index of the biggest cell to the left of 'idx' that
/// isn't activated yet.
fn find_furthest_left_unactivated_cell_from_idx(
    bank: &Battery,
    furthest_left_unactivated_cell_idx: usize,
    right_idx: usize,
) -> usize {
    // iterating leftwards, starting at 'right_idx', to the cell right of 'furthest_left_unactivated_cell_idx...
    let max_idx = (right_idx..furthest_left_unactivated_cell_idx + 1)
        .max_by_key(|i| get_digit_at_n(bank, *i))
        .expect("There has to be a cell that isn't activated yet...");

    max_idx
}

/// By turning on `on_at_once` cells in a `Battery`, what's the most joltage that can be produced?
fn calc_biggest_joltage_possible_for_battery(battery: &Battery, on_at_once: u64) -> u64 {
    // Iterate through the rightmost `on_at_once` cells.  Starting with the
    // leftmost of those cells, find the cell furthest to the left with the
    // highest joltage. Turn that cell on by returning it inside the map(), so
    // that the map() accumlates all the cells we turned on

    let mut furthest_left_unactivated_cell_idx = battery.len() - 1;

    let on_switch_indexes: Vec<usize> = (0..on_at_once as usize)
        .rev()
        .map(|idx| {
            let idx_of_highest_joltage_cell_available_to_the_left =
                find_furthest_left_unactivated_cell_from_idx(
                    battery,
                    furthest_left_unactivated_cell_idx,
                    idx,
                );

            // anti overflow for last digit
            if idx_of_highest_joltage_cell_available_to_the_left > 0 {
                furthest_left_unactivated_cell_idx =
                    idx_of_highest_joltage_cell_available_to_the_left - 1;
            }

            idx_of_highest_joltage_cell_available_to_the_left
        })
        .collect();

    calc_joltage_from_on_cells(battery, on_switch_indexes)
}

fn parse_input(input_str: &str) -> anyhow::Result<Vec<Battery>> {
    let ret: Vec<Battery> = input_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or_else(|| AocError::ParseError(format!("Invalid digit: {}", ch)))
                })
                .collect::<Result<Vec<u8>, AocError>>()
        })
        .collect::<Result<Vec<Battery>, AocError>>()?;

    Ok(ret)
}

fn calc_sum_of_joltages_for_batteries(batteries: &[Battery], on_at_once: u64) -> u64 {
    batteries
        .iter()
        .map(|battery| calc_biggest_joltage_possible_for_battery(battery, on_at_once))
        .sum()
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Hello from Day 3!");

    let input_str = load_input_file_in_src_dir_to_string!("input.txt")?;
    let batteries: Vec<Battery> = parse_input(&input_str)?;

    let cells_on_at_once = 2;
    let total_joltage_part_1 =
        calc_sum_of_joltages_for_batteries(batteries.as_slice(), cells_on_at_once);

    info!("Total joltage part 1: {:?}", total_joltage_part_1);

    let cells_on_at_once = 12;
    let total_joltage_part_2 =
        calc_sum_of_joltages_for_batteries(batteries.as_slice(), cells_on_at_once);

    info!("Total joltage part 2: {:?}", total_joltage_part_2);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_digit_at_n() {
        let my_vec: Vec<u8> = vec![1, 2, 3];

        assert_eq!(3, get_digit_at_n(&my_vec, 0));
        assert_eq!(2, get_digit_at_n(&my_vec, 1));
        assert_eq!(1, get_digit_at_n(&my_vec, 2));
    }

    #[test]
    fn test_sample_input_part_1() {
        let input_str = load_input_file_in_src_dir_to_string!("test_input.txt").unwrap();
        let batteries = parse_input(&input_str).unwrap();
        let on_at_once = 2;
        let actual = calc_sum_of_joltages_for_batteries(batteries.as_slice(), on_at_once);

        let expected = 357;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sample_input_part_2() {
        let input_str = load_input_file_in_src_dir_to_string!("test_input.txt").unwrap();
        let batteries = parse_input(&input_str).unwrap();
        let on_at_once = 12;
        let actual = calc_sum_of_joltages_for_batteries(batteries.as_slice(), on_at_once);

        let expected = 3121910778619;
        assert_eq!(expected, actual);
    }
}

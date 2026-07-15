use aoc_lib::{AocError, load_input_file_in_src_dir_to_string};
use env_logger::Env;
use log::info;
use ndarray::prelude::*;
use std::collections::HashSet;

/// Advent of Code 2025 Day 4
/// <https://adventofcode.com/2025/day/4>
///
/// We're given a 2D array showing where paper rolls are
/// on the floor of a printing department.
///
/// A roll is accessible by forklift if there are fewer than
/// four adjacent rolls of paper in the 8 adjacent positions.
///
/// Part 1: Count how many paper rolls are accessible by forklift.
///
/// Part 2: Use the forklift to remove accessible paper rolls. How
///         many rolls of paper can be removed in total?

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CellType {
    Empty,
    Paper,
}

fn parse_input_str(input_str: &str) -> anyhow::Result<Array2<CellType>> {
    let num_rows = input_str.lines().count();
    let num_cols = if let Some(first_line) = input_str.lines().next() {
        first_line.len()
    } else {
        return Err(AocError::ParseError("Input file had no lines".to_string()).into());
    };

    let mut a = Array::from_elem((num_rows, num_cols), CellType::Empty);

    for (row_idx, line) in input_str.lines().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            if ch == '@' {
                a[[row_idx, col_idx]] = CellType::Paper;
            }
        }
    }

    Ok(a)
}

fn count_adjacent_rolls(arry: &Array2<CellType>, row: usize, col: usize) -> u32 {
    let mut acc = 0;

    let num_rows = arry.shape()[0];
    let num_cols = arry.shape()[1];

    if row > 0 && arry[[row - 1, col]] == CellType::Paper {
        acc += 1;
    }

    if row > 0 && col > 0 && arry[[row - 1, col - 1]] == CellType::Paper {
        acc += 1;
    }

    if row > 0 && col < num_cols - 1 && arry[[row - 1, col + 1]] == CellType::Paper {
        acc += 1;
    }

    if col > 0 && arry[[row, col - 1]] == CellType::Paper {
        acc += 1;
    }

    if col < num_cols - 1 && arry[[row, col + 1]] == CellType::Paper {
        acc += 1;
    }

    if row < num_rows - 1 && col > 0 && arry[[row + 1, col - 1]] == CellType::Paper {
        acc += 1;
    }

    if row < num_rows - 1 && arry[[row + 1, col]] == CellType::Paper {
        acc += 1;
    }

    if row < num_rows - 1 && col < num_cols - 1 && arry[[row + 1, col + 1]] == CellType::Paper {
        acc += 1;
    }

    acc
}

fn find_accessible_rolls(arry: &Array2<CellType>) -> HashSet<(usize, usize)> {
    let num_rows = arry.shape()[0];
    let num_cols = arry.shape()[1];

    let mut acc = HashSet::<(usize, usize)>::new();

    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            if arry[[row_idx, col_idx]] == CellType::Paper
                && count_adjacent_rolls(arry, row_idx, col_idx) < 4
            {
                acc.insert((row_idx, col_idx));
            }
        }
    }

    acc
}

fn count_accessible_rolls(arry: &Array2<CellType>) -> u32 {
    let accessible_rolls = find_accessible_rolls(arry);

    accessible_rolls.len() as u32
}

fn count_rolls(arry: &Array2<CellType>) -> u32 {
    let mut acc = 0;

    let num_rows = arry.shape()[0];
    let num_cols = arry.shape()[1];

    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            if arry[[row_idx, col_idx]] == CellType::Paper {
                acc += 1;
            }
        }
    }

    acc
}

fn remove_rolls(arry: &mut Array2<CellType>, rolls_to_remove: HashSet<(usize, usize)>) {
    for roll in rolls_to_remove {
        let row = roll.0;
        let col = roll.1;

        arry[[row, col]] = CellType::Empty;
    }
}

fn count_removable_rolls(arry: &mut Array2<CellType>) -> u32 {
    let mut last_count_of_rolls = count_rolls(arry);
    let mut num_rolls_removed = 0;

    loop {
        let accessible_rolls = find_accessible_rolls(arry);
        if accessible_rolls.is_empty() {
            break;
        }

        remove_rolls(arry, accessible_rolls);

        let new_count_of_rolls = count_rolls(arry);

        let num_rows_removed_this_iter = last_count_of_rolls - new_count_of_rolls;

        num_rolls_removed += num_rows_removed_this_iter;
        last_count_of_rolls = new_count_of_rolls;
    }

    num_rolls_removed
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Hello from Day 4!");

    let input_str = load_input_file_in_src_dir_to_string!("input.txt")?;

    let mut a = parse_input_str(&input_str)?;

    let num_accessible_rolls = count_accessible_rolls(&a);
    info!("Num accessible rows: {:?}", num_accessible_rolls);

    let num_removable_rows = count_removable_rolls(&mut a);
    info!("Num removable rows: {:?}", num_removable_rows);

    Ok(())
}

// part 1: 1602
// part 2: 9518

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        let input_str = load_input_file_in_src_dir_to_string!("test_input.txt").unwrap();
        let a = parse_input_str(&input_str).unwrap();
        let actual = count_accessible_rolls(&a);

        let expected = 13;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sample_input_part_2() {
        let input_str = load_input_file_in_src_dir_to_string!("test_input.txt").unwrap();
        let mut a = parse_input_str(&input_str).unwrap();
        let actual = count_removable_rolls(&mut a);

        let expected = 43;
        assert_eq!(expected, actual);
    }
}

use anyhow::Context;
use env_logger::Env;
use log::{debug, info};
use std::collections::HashSet;

use aoc_lib::AocError;

/// Advent of Code 2025 Day 2
/// <https://adventofcode.com/2025/day/2>
///
/// We're given some numeric ranges representing product IDs. Some numbers
/// in the ranges are invalid. For part 1 and part 2, we have to
/// add up all the invalid IDs
///
/// For part 1, an ID is invalid if it's made up of sequences of digits
/// repeated twice.
///
/// For part 2, an ID is invalid if it's made up of sequences of digits
/// repeated *at least* twice.
///
/// My approach was to generate all invalid IDs within a range. Rather
/// than check all numbers within a range to see if they're valid or not.

#[derive(Debug, PartialEq, Eq)]
/// A numeric range of values.
struct Range {
    /// The first number in the range.
    start: u64,
    /// The last number in the range.
    end: u64,
}

impl Range {
    /// Is `num` inside the `Range`?
    fn contains(&self, num: u64) -> bool {
        num >= self.start && num <= self.end
    }
}

/// Parse a `Range` from a string like "1188511880-1188511890"
fn parse_range(range_str: &str) -> anyhow::Result<Range> {
    let range_str_split = range_str
        .split('-')
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|_| AocError::ParseError(range_str.into()))?;

    let [start_int, end_int] = range_str_split.as_slice() else {
        return Err(AocError::ParseError(range_str.into()).into());
    };

    Ok(Range {
        start: *start_int,
        end: *end_int,
    })
}

/// How many digits are in number `x`?
fn get_num_digits(x: u64) -> u32 {
    if x == 0 { 1 } else { x.ilog10() + 1 }
}

/// Extend some numeric stem like '123' to some length `len` by repeating the
/// stem until it's `len` long. If `len` isn't divisible by the stem length,
/// returns 0.
fn repeat_stem_to_len(stem: u64, len: u32) -> u64 {
    let stem_len = get_num_digits(stem);

    // len has to be divisible by stem_len
    if !len.is_multiple_of(stem_len) {
        return 0;
    }

    // add up some powers of 10 of `stem` to the the result
    let repetitions = len / stem_len;
    (0..repetitions)
        .map(|i| {
            let exp = stem_len * i;
            stem * 10u64.pow(exp)
        })
        .sum()
}

/// Given some stem length `n`, generate all invalid IDs within a `Range` that
/// have that stem length.
///
/// The rules for invalid IDs are different for part 2, so a bool indicates that.
fn generate_invalid_ids_stem_len_n(range: &Range, n: u32, is_part_2: bool) -> HashSet<u64> {
    let mut ret = HashSet::<u64>::new();

    let range_len_start = get_num_digits(range.start);
    let range_len_end = get_num_digits(range.end);

    // The stem length 'n' has 10 numbers. Let's try and build invalid IDs out
    // of each of them.
    let decade_start = 10u64.pow(n - 1);
    let decade_end = 10u64.pow(n) - 1;

    for stem in decade_start..=decade_end {
        // For part 2, for each stem in the decade, try extending the stem to
        // the length of the range start, and to the length of the range
        // end. Looking at the input data, the range end is never more than one
        // length longer than the range start.
        if is_part_2 {
            let candidate_1 = repeat_stem_to_len(stem, range_len_start);

            // First try extending 'stem' to the length of 'range.start'
            if range.contains(candidate_1) && candidate_1 > 10 {
                debug!("Invalid ID: {:?}", candidate_1);
                ret.insert(candidate_1);
            }

            // Also try extending 'stem' to the length of 'range.end'
            if range_len_start != range_len_end {
                let candidate_2 = repeat_stem_to_len(stem, range_len_end);
                if range.contains(candidate_2) {
                    debug!("Invalid ID: {:?}", candidate_2);
                    ret.insert(candidate_2);
                }
            }
        } else {
            // in part 1, you can only dupicate the stem once.
            let candidate = stem * 10u64.pow(n) + stem;
            if range.contains(candidate) {
                ret.insert(candidate);
            }
        }
    }

    ret
}

/// This is the top level of the solution per Range. Get all the numbers within
/// a range that are 'invalid IDs'. The caller can then add them all up.
///
/// What numbers are considered invalid IDs or not is different for part 2, so
/// this is indicated by a parameter.
fn find_invalid_ids_for_range(range: &Range, is_part_2: bool) -> Vec<u64> {
    let mut acc = HashSet::<u64>::new();

    // This works by generating all invalid IDs for each "stem length" that'll
    // get repeated to generate invalid IDs within the `Range`

    // How many digits long is the range end?
    let num_digits_end = get_num_digits(range.end);

    // For each stem length possible...
    for stem_length in 1..=(num_digits_end / 2) {
        let set_m = generate_invalid_ids_stem_len_n(range, stem_length, is_part_2);
        acc.extend(set_m);
    }

    Vec::<u64>::from_iter(acc)
}

fn parse_input_str(input: &str) -> anyhow::Result<Vec<Range>> {
    input
        .split(",")
        .map(parse_range)
        .collect::<Result<Vec<Range>, _>>()
        .with_context(|| "It should parse.")
}

/// The top level of the solution.
fn calc_sum_of_invalid_ids_for_ranges(ranges: &[Range], is_part_2: bool) -> u64 {
    // For each range, add up the invliad IDs. And add up those sums. So, a sum
    // over a flat_map() can be used.
    ranges
        .iter()
        .flat_map(|r| find_invalid_ids_for_range(r, is_part_2))
        .sum()
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Hello from Day 2!");

    let input_str = aoc_lib::load_input_file_in_src_dir_to_string!("input.txt")?;
    let ranges = parse_input_str(&input_str)?;

    // Do parts 1 and 2 by iterating over the part numbers...
    for (part, is_part_2) in [(1, false), (2, true)] {
        let sum_of_invalid_ids = calc_sum_of_invalid_ids_for_ranges(ranges.as_slice(), is_part_2);
        info!("Part {:?}: {:?}", part, sum_of_invalid_ids);
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_range() {
        let range_str = "123-456";
        let expected = Range {
            start: 123,
            end: 456,
        };
        let actual = parse_range(range_str).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_num_digits_even() {
        let num = 123456;
        let expected = 6;
        let actual = get_num_digits(num);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_num_digits_odd() {
        let num = 12345;
        let expected = 5;
        let actual = get_num_digits(num);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_extend_stem_to_len_1() {
        let stem = 123;
        let len = 6;

        let expected = 123123;
        let actual = repeat_stem_to_len(stem, len);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_extend_stem_to_len_2() {
        let stem = 12;
        let len = 6;

        let expected = 121212;
        let actual = repeat_stem_to_len(stem, len);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_range_4_to_19() {
        // This range was wrong for my first attempt at part 2
        let range = Range { start: 4, end: 19 };

        let is_part_2 = true;
        let expected = vec![11];
        let actual = find_invalid_ids_for_range(&range, is_part_2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sample_input() {
        let input_str = aoc_lib::load_input_file_in_src_dir_to_string!("test_input.txt").unwrap();
        let ranges = parse_input_str(&input_str).expect("Can parse input_str");

        let is_part_2 = false;
        assert_eq!(
            1227775554,
            calc_sum_of_invalid_ids_for_ranges(ranges.as_slice(), is_part_2)
        );

        let is_part_2 = true;
        assert_eq!(
            4174379265,
            calc_sum_of_invalid_ids_for_ranges(ranges.as_slice(), is_part_2)
        );
    }
}

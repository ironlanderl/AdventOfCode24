use core::num;
use std::iter::repeat_n;

use crate::utils::read_file;
use itertools::Itertools;
use cached::proc_macro::cached;


fn get_data(input_file: &str) -> Vec<i64> {
    let mut stones: Vec<i64> = vec![];
    let content = read_file(input_file.to_string());

    for line in content.lines() {
        for number in line.split(' ') {
            stones.push(number.parse::<i64>().unwrap());
        }
    }
    stones
}

#[cached]
fn score(number: i64, blinks: i64) -> i64 {
    if blinks == 0 {
        return 1;
    }

    if number == 0 {
        return score(1, blinks - 1);
    } else if (number.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        let left_half = number / 10_i64.pow((number.checked_ilog10().unwrap_or(0) + 1) / 2);
        let right_half = number % 10_i64.pow((number.checked_ilog10().unwrap_or(0) + 1) / 2);

        return score(left_half, blinks - 1) + score(right_half, blinks - 1);
    } else {
        return score(number * 2024, blinks - 1);
    }
}

fn solve_p1(stones: Vec<i64>) -> i64 {
    let mut solution: i64 = 0;
    for stone in stones {
        solution += score(stone, 25);
    }
    solution
}

fn solve_p2(stones: Vec<i64>) -> i64 {
    let mut solution: i64 = 0;
    for stone in stones {
        solution += score(stone, 75);
    }
    solution
}

pub fn day11a(input_file: &str) -> i64 {
    let map = get_data(input_file);
    let solution = solve_p1(map);
    println!("Solution: {:?}", solution);
    solution
}

pub fn day11b(input_file: &str) -> i64 {
    let map = get_data(input_file);
    let solution = solve_p2(map);
    println!("Solution: {:?}", solution);
    solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::*;

    #[test]
    fn test_part1() {
        env_logger::init();
        assert_eq!(day11a("inputs/day11ex.txt"), 55312); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2() {
        let input = "your test input here";
        assert_eq!(day11b("inputs/day10ex.txt"), 81); // Replace 0 with the expected result
    }
}

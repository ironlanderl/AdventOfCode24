use core::num;
use std::{iter::repeat_n, ops::Index};

use crate::utils::read_file;
use itertools::Itertools;

fn get_data(input_file: &str) -> Vec<i64> {
    let mut FILEMAP: Vec<i64> = vec![];
    let content = read_file(input_file.to_string());

    for line in content.lines() {
        for number in line.chars() {
            FILEMAP.push(number.to_string().parse::<i64>().unwrap());
        }
    }
    FILEMAP
}

fn decompress(FILEMAP: Vec<i64>) -> Vec<i64> {
    let mut RAM: Vec<i64> = vec![];
    let mut second_index: i64 = 0;

    for (index, number) in FILEMAP.iter().enumerate() {
        // indexes 0,2,4, etc.. Files
        // indexes 1,3,5, etc.. Spaces
        if index % 2 == 0 {
            for _ in 0..*number {
                RAM.push(second_index);
            }
            second_index += 1;
        } else {
            for _ in 0..*number {
                RAM.push(-1);
            }
        }
    }
    RAM
}

fn defragment(RAM: &mut Vec<i64>) {
    let mut left_index: usize = 0;
    let mut right_index: usize = RAM.len() - 1;

    // Right to left, move everything to the left
    while right_index > left_index {
        // 1st check -> right index on a number
        if RAM[right_index] != -1 {
            // Check if left index is on an empty spot
            while RAM[left_index] != -1 && left_index < right_index {
                left_index += 1;
            }
            if RAM[left_index] == -1 {
                // Move
                RAM[left_index] = RAM[right_index];
                RAM[right_index] = -1;
            }
        }
        right_index -= 1;
    }
}

fn checksum(RAM: Vec<i64>) -> i64 {
    RAM.into_iter()
        .enumerate()
        .map(|(index, element)| {
            if element != -1 {
                element * index as i64
            }
            else {
                0
            }
        })
        .sum()
}

fn solve_p1(FILEMAP: Vec<i64>) -> i64 {
    let mut RAM = decompress(FILEMAP);
    defragment(&mut RAM);
    checksum(RAM)
}

fn solve_p2(stones: Vec<i64>) -> i64 {
    let mut solution: i64 = 0;

    solution
}

pub fn day9a(input_file: &str) -> i64 {
    let map = get_data(input_file);
    let solution = solve_p1(map);
    println!("Solution: {:?}", solution);
    solution
}

pub fn day9b(input_file: &str) -> i64 {
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
        assert_eq!(day9a("inputs/day9ex.txt"), 1928); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2() {
        let input = "your test input here";
        assert_eq!(day9b("inputs/day9ex.txt"), 81); // Replace 0 with the expected result
    }
}

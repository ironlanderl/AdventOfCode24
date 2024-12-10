use std::iter::repeat_n;

use crate::utils::read_file;
use itertools::Itertools;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

fn get_data(input_file: &str) -> Vec<Vec<i64>> {
    let mut map: Vec<Vec<i64>> = vec![];
    let content = read_file(input_file.to_string());

    for line in content.lines() {
        /* sol: eq */
        let mut row: Vec<i64> = vec![];

        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as i64);
        }
        map.push(row);
    }
    map
}

fn count_nines_without_dups(mut x: usize, mut y: usize, map: &Vec<Vec<i64>>) -> i64 {
    let mut stack = vec![(x, y)];
    let mut count = 0;
    while let Some((cx, cy)) = stack.pop() {

        let current_number = map[cy][cx];

        if current_number == 9 {
            count += 1;
        }

        if cy > 0 && map[cy - 1][cx] == current_number + 1 {
            stack.push((cx, cy - 1));
        }
        if cy < map.len() - 1 && map[cy + 1][cx] == current_number + 1 {
            stack.push((cx, cy + 1));
        }
        if cx > 0 && map[cy][cx - 1] == current_number + 1 {
            stack.push((cx - 1, cy));
        }
        if cx < map[0].len() - 1 && map[cy][cx + 1] == current_number + 1 {
            stack.push((cx + 1, cy));
        }
    }

    count
}

fn count_nines(mut x: usize, mut y: usize, map: &Vec<Vec<i64>>) -> i64 {
    let mut stack = vec![(x, y)];
    let mut count = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    while let Some((cx, cy)) = stack.pop() {
        if visited[cy][cx] {
            continue;
        }
        visited[cy][cx] = true;

        let current_number = map[cy][cx];

        if current_number == 9 {
            count += 1;
        }

        if cy > 0 && map[cy - 1][cx] == current_number + 1 && !visited[cy - 1][cx] {
            stack.push((cx, cy - 1));
        }
        if cy < map.len() - 1 && map[cy + 1][cx] == current_number + 1 && !visited[cy + 1][cx] {
            stack.push((cx, cy + 1));
        }
        if cx > 0 && map[cy][cx - 1] == current_number + 1 && !visited[cy][cx - 1] {
            stack.push((cx - 1, cy));
        }
        if cx < map[0].len() - 1 && map[cy][cx + 1] == current_number + 1 && !visited[cy][cx + 1] {
            stack.push((cx + 1, cy));
        }
    }

    count
}

fn solve_p1(map: Vec<Vec<i64>>) -> i64 {
    let mut solution: i64 = 0;
    let ok_points: Vec<Point> = vec![];

    for (i, row) in map.iter().enumerate() {
        for (j, element) in row.into_iter().enumerate() {
            if *element == 0{

                solution += count_nines(j,i, &map);
            }

        }
    }

    solution
}

fn solve_p2(map: Vec<Vec<i64>>) -> i64 {
    let mut solution: i64 = 0;
    let ok_points: Vec<Point> = vec![];

    for (i, row) in map.iter().enumerate() {
        for (j, element) in row.into_iter().enumerate() {
            if *element == 0{

                solution += count_nines_without_dups(j,i, &map);
            }

        }
    }

    solution
}

pub fn day10a(input_file: &str) -> i64 {
    let map = get_data(input_file);
    let solution = solve_p1(map);
    println!("Solution: {:?}", solution);
    solution
}

pub fn day10b(input_file: &str) -> i64 {
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
        assert_eq!(day10a("inputs/day10ex.txt"), 36); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2() {
        let input = "your test input here";
        assert_eq!(day10b("inputs/day10ex.txt"), 81); // Replace 0 with the expected result
    }
}

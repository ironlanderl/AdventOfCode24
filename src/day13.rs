use core::num;
use std::{alloc::System, iter::repeat_n, ops::Index};

use crate::utils::read_file;
use good_lp::{constraint, default_solver, highs, variables, Solution, SolverModel};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct CoordinateData {
    name: String,
    x: i32,
    y: i32,
}

fn extract_coordinates(input: &str) -> Vec<CoordinateData> {
    let re = Regex::new(r"(?P<name>Button [AB]|Prize): X[+=](?P<x>\d+), Y[+=](?P<y>\d+)").unwrap();
    let mut results = Vec::new();

    for caps in re.captures_iter(input) {
        let name = caps.name("name").unwrap().as_str().to_string();
        let x: i32 = caps.name("x").unwrap().as_str().parse().unwrap();
        let y: i32 = caps.name("y").unwrap().as_str().parse().unwrap();

        results.push(CoordinateData { name, x, y });
    }

    results
}

fn solve_p1(input_file: &str) -> i64 {
    let content = read_file(input_file.to_string());
    let mut tokens: i64 = 0;

    for group in content.split("\n\n") {
        /*  data[0]: A
            data[1]: B
            data[2]: Obj
        */
        let data = extract_coordinates(group);

        variables! {
            vars:
                // Numero di volte che premo A
                A (integer) >= 0;
                // Numero di volte che premo B
                B (integer) >= 0;
        }
        let mut model = vars
            .minimise(3 * A + 1 * B) // Numero di token
            .using(highs)
            // Asse X
            .with(constraint!(A * data[0].x + B * data[1].x == data[2].x))
            // Asse Y
            .with(constraint!(A * data[0].y + B * data[1].y == data[2].y));

        model.set_verbose(false);

        let result = model.solve();

        match result {
            Ok(solution) => {
                // println!("a={}   b={}", solution.value(A), solution.value(B));
                let A2 = solution.value(A);
                let B2 = solution.value(B);

                let A3 = A2.round() as i64;
                let B3 = B2.round() as i64;

                if A2.fract() != 0.0 {
                    println!("PANIC!");
                }
                if B2.fract() != 0.0 {
                    println!("PANIC!");
                }

                tokens += 3 * A2 as i64 + B2 as i64;
            }
            Err(_) => {
                // println!("Problema impossibile")
            }
        }
    }

    tokens
}

fn solve_p2(input_file: &str) -> i128 {
    let content = read_file(input_file.to_string());
    let mut tokens: i128 = 0;

    for group in content.split("\n\n") {
        /*  data[0]: A
            data[1]: B
            data[2]: Obj
        */
        let data = extract_coordinates(group);

        let constr1:f64 = data[2].x as f64 + 10000000000000.0;
        let constr2:f64 = data[2].y as f64 + 10000000000000.0;

        variables! {
            vars:
                // Numero di volte che premo X
                A (integer) >= 0;
                // Numero di volte che premo Y
                B (integer) >= 0;
        }
        let mut model = vars
            .minimise(3 * A + 1 * B) // Numero di token
            .using(highs)
            // Asse X
            .with(constraint!(
                A * data[0].x + B * data[1].x == constr1
            ))
            // Asse Y
            .with(constraint!(
                A * data[0].y + B * data[1].y == constr2
            ));

        model.set_verbose(false);

        let result = model.solve();

        match result {
            Ok(solution) => {
                // println!("a={}   b={}", solution.value(A), solution.value(B));
                let A2 = solution.value(A);
                let B2 = solution.value(B);

                let A3 = A2.round() as i64;
                let B3 = B2.round() as i64;

                if A2.fract() != 0.0 {
                    println!("PANIC!");
                }
                if B2.fract() != 0.0 {
                    println!("PANIC!");
                }

                tokens += 3 * A2 as i128 + B2 as i128;
            }
            Err(_) => {
                // println!("Problema impossibile")
            }
        }
    }

    tokens
}

pub fn day13a(input_file: &str) -> i64 {
    let solution = solve_p1(input_file);
    println!("Solution: {:?}", solution);
    solution
}

pub fn day13b(input_file: &str) -> i128 {
    let solution = solve_p2(input_file);
    println!("Solution: {:?}", solution);
    solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::*;

    #[test]
    fn test_part1_1() {
        env_logger::init();
        assert_eq!(day13a("inputs/day13ex.txt"), 480); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(day13b("inputs/day13ex.txt"), 80); // Replace 0 with the expected result
    }
}

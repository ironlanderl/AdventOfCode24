use std::iter::repeat_n;

use crate::utils::read_file;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OperationTypes {
    SUM,
    MUL,
    CON,
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}

fn get_data(input_file: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
    let mut solutions: Vec<i64> = vec![];
    let mut equations: Vec<Vec<i64>> = vec![];
    let content = read_file(input_file.to_string());

    for line in content.lines() {
        /* sol: eq */
        let spl: Vec<&str> = line.split(':').collect();

        solutions.push(spl[0].parse::<i64>().unwrap());
        let mut equation: Vec<i64> = vec![];

        for number in spl[1].split(' ') {
            if number != "" {
                equation.push(number.parse::<i64>().unwrap());
            }
        }

        equations.push(equation);
    }

    (solutions, equations)
}

fn solve_p1(solutions: Vec<i64>, equations: Vec<Vec<i64>>) -> i64 {
    let mut solution: i64 = 0;

    for (i, equation) in equations.iter().enumerate() {
        let possible_operations = vec![OperationTypes::SUM, OperationTypes::MUL].into_iter();
        let possible_ops =
            repeat_n(possible_operations, equation.len() - 1).multi_cartesian_product();

        for (j, stuff) in possible_ops.enumerate() {
            let mut res: i64 = 0;
            for (k, operator) in stuff.into_iter().enumerate() {
                if k == 0 {
                    match operator {
                        OperationTypes::SUM => res += equation[k] + equation[k + 1],
                        OperationTypes::MUL => res += equation[k] * equation[k + 1],
                        OperationTypes::CON => todo!(),
                    }
                } else {
                    match operator {
                        OperationTypes::SUM => res += equation[k + 1],
                        OperationTypes::MUL => res *= equation[k + 1],
                        OperationTypes::CON => todo!(),
                    }
                }
            }
            if res == solutions[i] {
                solution += res;
                break;
            }
        }

        // println!("Test: {:?}", maxj);
    }

    solution
}

fn solve_p2(target_solutions: Vec<i64>, equations: Vec<Vec<i64>>) -> i64 {
    let mut total_solution: i64 = 0;

    for (index, mut equation) in equations.into_iter().enumerate() {
        let possible_operations = vec![
            OperationTypes::SUM,
            OperationTypes::MUL,
            OperationTypes::CON,
        ]
        .into_iter();
        let possible_combinations =
            repeat_n(possible_operations, equation.len() - 1).multi_cartesian_product();

        for operations in possible_combinations {
            let mut result: i64 = 0;
            for (op_index, operation) in operations.into_iter().enumerate() {
                if op_index == 0 {
                    match operation {
                        OperationTypes::SUM => {
                            result += equation[op_index] + equation[op_index + 1];
                        }
                        OperationTypes::MUL => {
                            result += equation[op_index] * equation[op_index + 1];
                        }
                        OperationTypes::CON => {
                            result = concat(equation[op_index], equation[op_index + 1]);
                        }
                    }
                } else {
                    match operation {
                        OperationTypes::SUM => {
                            result += equation[op_index + 1];
                        }
                        OperationTypes::MUL => {
                            result *= equation[op_index + 1];
                        }
                        OperationTypes::CON => {
                            result = concat(result, equation[op_index + 1]);
                        }
                    }
                }
            }
            if result == target_solutions[index] {
                total_solution += result;
                break;
            }
        }
    }

    total_solution
}

pub fn day7a(input_file: &str) -> i64 {
    let (solutions, equations) = get_data(input_file);
    let solution = solve_p1(solutions, equations);
    println!("Solution: {:?}", solution);
    solution
}

pub fn day7b(input_file: &str) -> i64 {
    let (solutions, equations) = get_data(input_file);
    let solution = solve_p2(solutions, equations);
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
        assert_eq!(day7a("inputs/day7ex.txt"), 3749); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2() {
        let input = "your test input here";
        assert_eq!(day7b("inputs/day7ex.txt"), 11387); // Replace 0 with the expected result
    }
}

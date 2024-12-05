use core::num;
use std::collections::HashMap;
use itertools::Itertools;
use progressing::{
    // The underlying Trait
    Baring,
    // Just handy names for the examples below
    bernoulli::Bar as BernoulliBar,
    clamping::Bar as ClampingBar,
    mapping::Bar as MappingBar,
};

fn factorial(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1)
    }
}

use crate::utils::read_file;

pub fn day5a(filename: &str) -> i64 {
    let (constraints, updates) = get_data(filename);
    let solution = solve_p1(constraints, updates);
    println!("Solution: {:?}", solution);
    solution
}

pub fn day5b(filename: &str) -> i64 {
    let (constraints, updates) = get_data(filename);
    let solution = solve_p2(constraints, updates);
    println!("Solution: {:?}", solution);
    solution
}

fn is_number_after(source: i64, target: i64, arr: &Vec<i64>) -> bool {
    let source_index = arr.iter().position(|p| *p == source).unwrap();
    let target_index = arr.iter().position(|p| *p == target).unwrap();
    source_index < target_index
}

fn get_data(filename: &str) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let content = read_file(filename.to_string());
    let mut contraints: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut updates: Vec<Vec<i64>> = vec![];

    for line in content.lines() {
        // Handle constraints
        if line.contains('|') {
            // First number is the page
            // Second is which page it should be printed before
            let numbers: Vec<i64> = line
                .split('|')
                .map(|f| f.parse::<i64>().ok().unwrap())
                .collect();
            contraints
                .entry(numbers[0])
                .and_modify(|f| f.push(numbers[1]))
                .or_insert(vec![numbers[1]]);
        } else if line.contains(',') {
            updates.push(
                line.split(',')
                    .map(|f| f.parse::<i64>().ok().unwrap())
                    .collect(),
            );
        }
    }

    (contraints, updates)
}

fn is_update_valid(update: &Vec<i64>, constraints: &HashMap<i64, Vec<i64>>) -> bool {
    for &source_page in update {
        if let Some(forbidden_pages) = constraints.get(&source_page) {
            for &forbidden_page in forbidden_pages {
                // Only check if forbidden page is also in this update
                if update.contains(&forbidden_page) {
                    // If forbidden page comes before source page, this update is invalid
                    if is_number_after(forbidden_page, source_page, update) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn solve_p1(constraints: HashMap<i64, Vec<i64>>, updates: Vec<Vec<i64>>) -> i64 {
    let mut ok_updates: Vec<Vec<i64>> = vec![];
    
    for update in updates {
        if is_update_valid(&update, &constraints) {
            ok_updates.push(update);
        }
    }

    ok_updates.iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn solve_p2(constraints: HashMap<i64, Vec<i64>>, updates: Vec<Vec<i64>>) -> i64 {
    let mut not_ok_updates: Vec<Vec<i64>> = vec![];
    let mut ok_updates: Vec<Vec<i64>> = vec![];
    let mut cur_per: i64 = 0;
    
    for update in updates {
        if !is_update_valid(&update, &constraints) {
            not_ok_updates.push(update);
        }
    }

    let total_updates = not_ok_updates.len();
    let mut progress_bar = BernoulliBar::with_goal(total_updates).timed();
    for (idx, invalid_update) in not_ok_updates.iter().enumerate() {
        let permutations = invalid_update.into_iter().permutations(invalid_update.len());
        progress_bar.set(idx + 1);
        cur_per = 0;
        for permutation in permutations {
            cur_per += 1;
            let comb_copy = permutation.clone();
            if is_update_valid(&permutation.into_iter().copied().collect(), &constraints) {
                ok_updates.push(comb_copy.into_iter().copied().collect());
                break;
            }
            print!("\r({}/{}){}", cur_per, factorial(invalid_update.len() as u128), progress_bar);
        }
    }
    println!();
    // Implement the rest of the logic for solve_p2
    ok_updates.iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = day5a("inputs/day5ex.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        let result = day5b("inputs/day5ex.txt");
        assert_eq!(result, 123);
    }
}


use core::num;
use std::{iter::repeat_n, ops::Index};

use crate::utils::read_file;
use itertools::Itertools;

fn get_data(input_file: &str) -> Vec<Vec<char>> {
    let mut TileMap: Vec<Vec<char>> = vec![];
    let content = read_file(input_file.to_string());

    for line in content.lines() {
        let mut TileLine: Vec<char> = vec![];
        for number in line.chars() {
            TileLine.push(number);
        }
        TileMap.push(TileLine);
    }
    TileMap
}

fn TileMapContains(TileMap: &Vec<Vec<char>>, element: char) -> bool{
    TileMap.iter().flat_map(|line| line.iter()).contains(&element)
}


fn dfs(TileMap: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, element: char) -> Vec<(usize, usize)> {
    let mut stack = vec![(i, j)];
    let mut region = vec![];

    while let Some((x, y)) = stack.pop() {
        if x >= TileMap.len() || y >= TileMap[0].len() || visited[x][y] || TileMap[x][y] != element {
            continue;
        }

        visited[x][y] = true;
        region.push((x, y));

        if x > 0 {
            stack.push((x - 1, y));
        }
        if x < TileMap.len() - 1 {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y < TileMap[0].len() - 1 {
            stack.push((x, y + 1));
        }
    }

    region
}

fn get_touching_regions(TileMap: &Vec<Vec<char>>, element: char) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; TileMap[0].len()]; TileMap.len()];
    let mut regions = vec![];

    for i in 0..TileMap.len() {
        for j in 0..TileMap[0].len() {
            if TileMap[i][j] == element && !visited[i][j] {
                regions.push(dfs(TileMap, &mut visited, i, j, element));
            }
        }
    }

    regions
}

fn calculate_perimeter(region: &Vec<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(x, y) in region {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if !region.contains(&(nx as usize, ny as usize)) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calculate_sides(region: &Vec<(usize, usize)>) -> usize {
    let mut sides = 0;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..region.len() {
        let (x, y) = region[i];
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if !region.contains(&(nx as usize, ny as usize)) {
                sides += 1;
            }
        }
    }

    sides
}

fn calculate_cost(regions: Vec<Vec<(usize, usize)>>) -> i64{
    let mut cost: i64 = 0;

    for region in regions{
        let area = region.len();
        let perimeter = calculate_perimeter(&region);
        cost += (area * perimeter) as i64;
    }

    cost
}

fn calculate_cost2(regions: Vec<Vec<(usize, usize)>>) -> i64{
    let mut cost: i64 = 0;

    for region in regions{
        let area = region.len();
        let perimeter = calculate_sides(&region);
        cost += (area * perimeter) as i64;
    }

    cost
}

fn solve_p1(TileMap: &mut Vec<Vec<char>>) -> i64 {
    let mut solution: i64 = 0;
    
    let unique_regions: Vec<char> = TileMap.iter().flat_map(|line| line.iter()).unique().cloned().collect();

    // Probably not the best way
    for region in unique_regions{
        solution += calculate_cost(get_touching_regions(TileMap, region));
    }

    solution
}

fn solve_p2(TileMap: &mut Vec<Vec<char>>) -> i64 {
    let mut solution: i64 = 0;
    
    let unique_regions: Vec<char> = TileMap.iter().flat_map(|line| line.iter()).unique().cloned().collect();

    // Probably not the best way
    for region in unique_regions{
        solution += calculate_cost2(get_touching_regions(TileMap, region));
    }

    solution
}

pub fn day12a(input_file: &str) -> i64 {
    let mut map = get_data(input_file);
    let solution = solve_p1(&mut map);
    println!("Solution: {:?}", solution);
    solution
}

pub fn day12b(input_file: &str) -> i64 {
    let mut map = get_data(input_file);
    let solution = solve_p2(&mut map);
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
        assert_eq!(day12a("inputs/day12ex1.txt"), 140); // Replace 0 with the expected result
    }

    #[test]
    fn test_part1_2() {
        env_logger::init();
        assert_eq!(day12a("inputs/day12ex2.txt"), 1930); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2_1() {
        let input = "your test input here";
        assert_eq!(day12b("inputs/day12ex1.txt"), 80); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2_2() {
        let input = "your test input here";
        assert_eq!(day12b("inputs/day12ex2.txt"), 1206); // Replace 0 with the expected result
    }
}

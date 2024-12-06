use log::debug;

use crate::utils::read_file;
use std::io::{self, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
enum TileType {
    EMPTY,
    OBSTACLE,
    VISITED,
}

enum Rotation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Guard {
    pos_x: usize,
    pos_y: usize,
    rotation: Rotation,
}

impl Guard {
    fn print_map(&self, tile_map: &Vec<Vec<TileType>>) {
        let mut output = String::new();
        for row in tile_map {
            for tile in row {
                output.push(tile.pretty_print());
            }
            output.push('\n');
        }
        debug!("{}", output);
    }

    fn rotate(&mut self) {
        match self.rotation {
            Rotation::UP => self.rotation = Rotation::RIGHT,
            Rotation::RIGHT => self.rotation = Rotation::DOWN,
            Rotation::DOWN => self.rotation = Rotation::LEFT,
            Rotation::LEFT => self.rotation = Rotation::UP,
        }
    }

    fn check_if_going_outside(&self, tile_map: &Vec<Vec<TileType>>) -> bool {
        match self.rotation {
            Rotation::UP => self.pos_y == 0,
            Rotation::DOWN => self.pos_y == (tile_map[0].len() as i64 - 1).try_into().unwrap(),
            Rotation::LEFT => self.pos_x == 0,
            Rotation::RIGHT => self.pos_x == (tile_map.len() as i64 - 1).try_into().unwrap(),
        }
    }

    fn move_guard(&mut self, tile_map: &mut Vec<Vec<TileType>>) {
        match self.rotation {
            Rotation::UP => {
                // Check if up is occupied
                if tile_map[self.pos_y - 1][self.pos_x] == TileType::OBSTACLE {
                    self.rotate();
                    return;
                }
                // Not occupied. Move up and mark the old position as visited
                tile_map[self.pos_y][self.pos_x] = TileType::VISITED;
                self.pos_y -= 1;
            }
            Rotation::DOWN => {
                // Check if down is occupied
                if tile_map[self.pos_y + 1][self.pos_x] == TileType::OBSTACLE {
                    self.rotate();
                    return;
                }
                // Not occupied. Move down and mark the old position as visited
                tile_map[self.pos_y][self.pos_x] = TileType::VISITED;
                self.pos_y += 1;
            }
            Rotation::LEFT => {
                // Check if left is occupied
                if tile_map[self.pos_y][self.pos_x - 1] == TileType::OBSTACLE {
                    self.rotate();
                    return;
                }
                // Not occupied. Move left and mark the old position as visited
                tile_map[self.pos_y][self.pos_x] = TileType::VISITED;
                self.pos_x -= 1;
            }
            Rotation::RIGHT => {
                // Check if right is occupied
                if tile_map[self.pos_y][self.pos_x + 1] == TileType::OBSTACLE {
                    self.rotate();
                    return;
                }
                // Not occupied. Move right and mark the old position as visited
                tile_map[self.pos_y][self.pos_x] = TileType::VISITED;
                self.pos_x += 1;
            }
        }
    }
}

impl TileType {
    fn pretty_print(&self) -> char {
        match self {
            TileType::EMPTY => '.',
            TileType::OBSTACLE => '#',
            TileType::VISITED => 'X',
        }
    }
}

fn load_data(filename: &str) -> (Guard, Vec<Vec<TileType>>) {
    let content = read_file(filename.to_string());
    let mut TileMap: Vec<Vec<TileType>> = vec![];
    let mut guard: Guard = Guard {
        pos_x: 0,
        pos_y: 0,
        rotation: Rotation::UP,
    };

    for (x, line) in content.lines().enumerate() {
        let mut TileLine: Vec<TileType> = vec![];
        for (y, character) in line.chars().enumerate() {
            match character {
                '.' => TileLine.push(TileType::EMPTY),
                '#' => TileLine.push(TileType::OBSTACLE),
                '^' => {
                    TileLine.push(TileType::EMPTY);
                    guard = Guard {
                        pos_x: y,
                        pos_y: x,
                        rotation: Rotation::UP,
                    };
                }
                'v' => {
                    TileLine.push(TileType::EMPTY);
                    guard = Guard {
                        pos_x: y,
                        pos_y: x,
                        rotation: Rotation::DOWN,
                    };
                }
                '<' => {
                    TileLine.push(TileType::EMPTY);
                    guard = Guard {
                        pos_x: y,
                        pos_y: x,
                        rotation: Rotation::LEFT,
                    };
                }
                '>' => {
                    TileLine.push(TileType::EMPTY);
                    guard = Guard {
                        pos_x: y,
                        pos_y: x,
                        rotation: Rotation::RIGHT,
                    };
                }
                _ => println!("HUH? {:?}", character),
            }
        }
        TileMap.push(TileLine);
    }

    (guard, TileMap)
}

fn solve_p1(mut guard: Guard, mut tile_map: Vec<Vec<TileType>>) -> i64 {
    let mut visits: i64 = 0;

    while !guard.check_if_going_outside(&tile_map) {
        guard.move_guard(&mut tile_map);
        guard.print_map(&tile_map);
    }

    for tile_row in tile_map.iter() {
        for tile in tile_row.iter() {
            if *tile == TileType::VISITED {
                visits += 1;
            }
        }
    }

    // The loop ends 1 short
    visits += 1;

    visits
}

pub fn day6a(filename: &str) -> i64 {
    let (mut guard, mut tile_map) = load_data(filename);
    let solution = solve_p1(guard, tile_map);
    println!("Visits: {:?}", solution);
    solution
}

pub fn day6b(filename: &str) -> usize {
    // Your code for part 2 here
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::*;

    #[test]
    fn test_part1() {
        env_logger::init();
        assert_eq!(day6a("inputs/day6ex.txt"), 41); // Replace 0 with the expected result
    }

    #[test]
    fn test_part2() {
        let input = "your test input here";
        assert_eq!(1, 0); // Replace 0 with the expected result
    }
}

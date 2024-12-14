use crate::utils::read_file;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Robot{
    x: i64,
    y: i64,
    vx: i64,
    vy: i64
}

impl Robot {
    fn move_seconds(&mut self, seconds: i64, board_width: i64, board_height: i64) {
        self.x = (self.x + self.vx * seconds).rem_euclid(board_width);
        self.y = (self.y + self.vy * seconds).rem_euclid(board_height);
    }

    fn get_quadrant(&self, board_width: i64, board_height: i64) -> i64 {
        let half_width = board_width / 2;
        let half_height = board_height / 2;

        let in_middle_x = self.x == half_width || (board_width % 2 == 0 && self.x == half_width - 1);
        let in_middle_y = self.y == half_height || (board_height % 2 == 0 && self.y == half_height - 1);

        if in_middle_x || in_middle_y {
            return -1;
        }

        match (self.x < half_width, self.y < half_height) {
            (true, true) => 1,
            (false, true) => 2,
            (true, false) => 3,
            (false, false) => 4,
        }
    }

}

fn load_robots(content: String) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];
    let re = Regex::new(r"p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap();

    for line in content.lines(){
        for caps in re.captures_iter(line) {
            let px: i64 = caps.name("px").unwrap().as_str().parse().unwrap();
            let py: i64 = caps.name("py").unwrap().as_str().parse().unwrap();
            let vx: i64 = caps.name("vx").unwrap().as_str().parse().unwrap();
            let vy: i64 = caps.name("vy").unwrap().as_str().parse().unwrap();
    
            robots.push(Robot {
                x: px, y: py, vx: vx, vy: vy
            });
        }
    }

    robots
}

fn check_if_multiple_in_row(robots: Vec<Robot>, amount: i64, board_width: i64) -> bool {
    let mut positions = vec![0; board_width as usize];

    for robot in robots {
        positions[robot.x as usize] += 1;
    }

    for count in positions {
        if count >= amount {
            return true;
        }
    }

    false
}

fn print_grid(robots: &Vec<Robot>, board_width: i64, board_height: i64) {
    let mut grid = vec![vec!['.'; board_width as usize]; board_height as usize];
    
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

pub fn day14a(input_file: &str, seconds: i64, board_width: i64, board_height: i64) -> u64 {
    let mut quadrant1 = 0;
    let mut quadrant2 = 0;
    let mut quadrant3 = 0;
    let mut quadrant4 = 0;

    let content = read_file(input_file.to_string());
    let robots = load_robots(content);
    for mut robot in robots{
        robot.move_seconds(seconds, board_width, board_height);
        match robot.get_quadrant(board_width, board_height) {
            1 => quadrant1 += 1,
            2 => quadrant2 += 1,
            3 => quadrant3 += 1,
            4 => quadrant4 += 1,
            -1 => {},
            _ => panic!("Unexpected quadrant value"),
        }
    }

    println!("Solution: {}", quadrant1 * quadrant2 * quadrant3 * quadrant4);

    quadrant1 * quadrant2 * quadrant3 * quadrant4
}

pub fn day14b(input_file: &str, board_width: i64, board_height: i64) -> i32 {
    let content = read_file(input_file.to_string());
    let robots = load_robots(content);

    let mut robot_vec = robots;
    for _i in 0..100000 {
        let mut any_aligned = false;
        for robot in &mut robot_vec {
            robot.move_seconds(1, board_width, board_height);
        }
        if check_if_multiple_in_row(robot_vec.clone(), 20, board_width) {
            print_grid(&robot_vec, board_width, board_height);
            println!("Frame {}, Board ok? (y/n)", _i);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                return _i + 1;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(day14a("inputs/day14ex.txt", 100, 11, 7), 12);
    }

    /*#[test]
    fn test_part2() {
        assert_eq!(day14b(TEST_INPUT), 0);
    }*/
}
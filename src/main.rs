mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day10;
mod day11;

mod utils;

use day1::{day1a, day1b};
use day2::{day2a, day2b};
use day3::{day3a, day3b};
use day4::{day4a};
use day5::{day5a, day5b};
use day6::{day6a, day6b};
use day7::{day7a, day7b};
use day10::{day10a, day10b};
use day11::{day11a, day11b};

use log::{debug, error, log_enabled, info, Level};

fn main() {
    env_logger::init();
    println!("Day 1");
    //day1a();
    //day1b();
    println!("Day 2");
    //day2a();
    //day2b();
    println!("Day 3");
    //day3a();
    //day3b();
    println!("Day 4");
    //day4a();
    println!("Day 5");
    //day5a("inputs/day5.txt");
    //day5b("inputs/day5.txt");
    println!("Day 6");
    //day6a("inputs/day6.txt");
    //day6b("inputs/day6.txt");
    println!("Day 7");
    //day7a("inputs/day7.txt");
    //day7b("inputs/day7.txt");
    println!("Day 10");
    //day10a("inputs/day10.txt");
    //day10b("inputs/day10.txt");
    println!("Day 11");
    day11a("inputs/day11.txt");
    day11b("inputs/day11.txt");
}



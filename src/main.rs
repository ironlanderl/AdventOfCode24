mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod utils;

use day1::{day1a, day1b};
use day2::{day2a, day2b};
use day3::{day3a, day3b};
use day4::{day4a};
use day5::{day5a, day5b};

fn main() {
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
    day4a();
    println!("Day 5");
    day5a("inputs/day5.txt");
    day5b("inputs/day5.txt");
}

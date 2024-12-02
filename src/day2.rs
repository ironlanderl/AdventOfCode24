use crate::utils::read_file;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TYPE_OF_REPORT {
    DECREASING,
    INCREASING
}

fn is_safe_report(levels: &Vec<i64>) -> bool {
    // Check 1: levels are all either increasing or decreasing
    let mut levels_reversed = levels.clone();
    levels_reversed.reverse();
    if !levels.is_sorted() && !levels_reversed.is_sorted() {
        return false;
    }

    // Check 2: i and i+1 are the same -> unsafe
    for i in 0..levels.len() - 1 {
        if levels[i] == levels[i + 1] {
            return false;
        }

        let diff = levels[i] - levels[i + 1];
        if !(diff.abs() < 4 && diff.abs() > 0) {
            return false;
        }
    }

    true
}

pub fn day2a() {
    let content = read_file("inputs/day2.txt".to_string());
    let mut safe_reports: i64 = 0;

    // Loop through lines
    for report in content.lines() {
        let mut levels: Vec<i64> = vec![];
        let levels_str: Vec<&str> = report.split(" ").collect();
        // Convert from string slice to i64
        for level_str in levels_str {
            match level_str.parse::<i64>() {
                Ok(level) => levels.push(level),
                Err(e) => eprintln!("Failed to parse level: {}", e),
            }
        }

        if is_safe_report(&levels) {
            safe_reports += 1;
        }
    }

    println!("Report sicuri: {}", safe_reports);
}

pub fn day2b() {
    let content = read_file("inputs/day2.txt".to_string());
    let mut safe_reports: i64 = 0;

    // Loop through lines
    for report in content.lines() {
        let mut levels: Vec<i64> = vec![];
        let levels_str: Vec<&str> = report.split(" ").collect();
        // Convert from string slice to i64
        for level_str in levels_str {
            match level_str.parse::<i64>() {
                Ok(level) => levels.push(level),
                Err(e) => eprintln!("Failed to parse level: {}", e),
            }
        }

        if is_safe_report(&levels) {
            safe_reports += 1;
        }
        else{
            // Welp. Time to bruteforce this shit
            for i in 0..levels.len(){
                let mut levels_clone = levels.clone();
                // Remove i number
                levels_clone.remove(i);

                if is_safe_report(&levels_clone){
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!("Report sicuri: {}", safe_reports);
}

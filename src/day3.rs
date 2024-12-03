use crate::utils::read_file;
use regex::Regex;

fn calculate_mults(mults: Vec<&str>) -> i64 {
    // Extract the numbers in '1' and '2'
    let mults_reg_val = Regex::new(r"(?<val1>\d+),(?<val2>\d+)").unwrap();
    let mut total: i64 = 0;

    for mult in mults {
        let values: Vec<(i64, i64)> = mults_reg_val
            .captures_iter(mult)
            .map(|caps| {
                // The unwraps are okay because every capture group must match if the whole
                // regex matches, and in this context, we know we have a match.
                //
                // Note that we use `caps.name("y").unwrap().as_str()` instead of
                // `&caps["y"]` because the lifetime of the former is the same as the
                // lifetime of `hay` above, but the lifetime of the latter is tied to the
                // lifetime of `caps` due to how the `Index` trait is defined.
                let num1 = caps
                    .name("val1")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .ok()
                    .unwrap();
                let num2 = caps
                    .name("val2")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .ok()
                    .unwrap();
                (num1, num2)
            })
            .collect();

        //println!("Huh? {:?}", values);

        total += values[0].0 * values[0].1
    }
    total
}

pub fn day3a() {
    let content = read_file("inputs/day3.txt".to_string());

    // Extract conly mul(number,number)
    let mults_reg = Regex::new(r"(?m)mul\(\d+,\d+\)").unwrap();

    let mults: Vec<&str> = mults_reg.find_iter(&content).map(|m| m.as_str()).collect();
    //println!("{:?}", mults);

    let total: i64 = calculate_mults(mults);

    println!("Total: {:?}", total);
}

pub fn day3b() {
    let mut content = read_file("inputs/day3.txt".to_string());

    // Make the content single lined
    content = content.replace('\n', "");

    // Welp. It's fucking absurd regex time.
    // Remove everything between don't and do.
    //println!("Content before regex: {:?}", content);
    let cleanup_reg = Regex::new(r"don't\(\)[\S\s]*?(do\(\)|$)").unwrap();
    //println!("Regex: {:?}", cleanup_reg);
    content = cleanup_reg.replace_all(&content, "").to_string();
    //println!("Content after regex: {:?}", content);

    // Extract only mul(number,number)
    let mults_reg = Regex::new(r"(?m)mul\(\d+,\d+\)").unwrap();

    let mults: Vec<&str> = mults_reg.find_iter(&content).map(|m| m.as_str()).collect();
    //println!("{:?}", mults);

    let total: i64 = calculate_mults(mults);

    println!("Total: {:?}", total);
}

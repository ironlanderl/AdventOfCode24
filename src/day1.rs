use crate::utils::read_file;

pub fn day1a(){
    let contents = read_file("inputs/day1.txt".to_string());
    let (mut col1, mut  col2) = load_matrix_to_variable(contents);
    let mut sum: i64 = 0;

    // Data loaded. Sort the columns from smallest to biggest
    col1.sort_unstable();
    col2.sort_unstable();

    // Loop
    for i in 0..col1.len(){
        sum += (col1[i] - col2[i]).abs();
    }

    // print distance
    println!("Total distance: {}", sum);
    
}

pub fn day1b(){
    let contents = read_file("inputs/day1.txt".to_string());
    let (col1, col2) = load_matrix_to_variable(contents);
    let mut sum: i64 = 0;

    // Data loaded. No need to sort this time

    // Loop through col1
    for location in col1{
        // Add the current location multiplied by it's appearence in the second list
        sum += location * col2.iter().filter(|&&x| x == location).count() as i64;
    }

    // print distance
    println!("Total distance: {}", sum);
    
}

fn load_matrix_to_variable(input_matrix: String) -> (Vec<i64>, Vec<i64>) {
    let mut col1: Vec<i64> = vec![];
    let mut col2: Vec<i64> = vec![];

    for line in input_matrix.lines() {
        let split_iterator: std::str::Split<'_, char> = line.split(' ');

        for split in split_iterator {
            // Skip empty elements. Should only happen after the last ' '
            if split.trim().is_empty() {
                continue;
            }

            let tmp2 = split.trim().parse::<i64>();

            if tmp2.is_err() {
                println!("Error adding value: {:?}", tmp2);
            } else { 
                let unwrapval = tmp2.unwrap(); // Ottiene il valore del parse
                if col1.len() > col2.len(){
                    // Col 1 has 1 more element, meaning we add to col2
                    col2.push(unwrapval);
                }
                else{
                    col1.push(unwrapval);
                }
            }
        }
    }

    (col1, col2)

}
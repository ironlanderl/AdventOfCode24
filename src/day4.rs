use crate::utils::read_file;

fn check_horizontal(
    content: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    reversed: Option<bool>,
) -> bool {
    //println!("------------");
    //println!("Content[row][column]: {:?}", content[row][column]);
    if reversed.unwrap() {
        if column as i64 - 3 < 0 {
            return false;
        }
        content[row][column] == 'X'
            && content[row][column - 1] == 'M'
            && content[row][column - 2] == 'A'
            && content[row][column - 3] == 'S'
    } else {
        if column + 3 >= content[row].len() {
            return false;
        }
        content[row][column] == 'X'
            && content[row][column + 1] == 'M'
            && content[row][column + 2] == 'A'
            && content[row][column + 3] == 'S'
    }
}

fn check_vertical(
    content: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    reversed: Option<bool>,
) -> bool {
    if reversed.unwrap() {
        if row as i64 - 3 < 0 {
            return false;
        }
        content[row][column] == 'X'
            && content[row - 1][column] == 'M'
            && content[row - 2][column] == 'A'
            && content[row - 3][column] == 'S'
    } else {
        if row + 3 > content.len() {
            return false;
        }
        content[row][column] == 'X'
            && content[row + 1][column] == 'M'
            && content[row + 2][column] == 'A'
            && content[row + 3][column] == 'S'
    }
}
fn check_diagonal_1_3(
    content: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    reversed: Option<bool>,
) -> bool {
    if reversed.unwrap() {
        if row as i64 - 3 < 0 || column as i64 - 3 < 0 {
            return false;
        }
        content[row][column] == 'X'
            && content[row - 1][column - 1] == 'M'
            && content[row - 2][column - 2] == 'A'
            && content[row - 3][column - 3] == 'S'
    } else {
        if row + 3 >= content.len() || column + 3 >= content[row].len() {
            return false;
        }
        content[row][column] == 'X'
            && content[row + 1][column + 1] == 'M'
            && content[row + 2][column + 2] == 'A'
            && content[row + 3][column + 3] == 'S'
    }
}

fn check_diagonal_2_4(
    content: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    reversed: Option<bool>,
) -> bool {
    if reversed.unwrap() {
        if row as i64 - 3 < 0 || column + 3 >= content[row].len() {
            return false;
        }
        content[row][column] == 'X'
            && content[row - 1][column + 1] == 'M'
            && content[row - 2][column + 2] == 'A'
            && content[row - 3][column + 3] == 'S'
    } else {
        if row + 3 >= content.len() || column as i64 - 3 < 0 {
            return false;
        }
        content[row][column] == 'X'
            && content[row + 1][column - 1] == 'M'
            && content[row + 2][column - 2] == 'A'
            && content[row + 3][column - 3] == 'S'
    }
}

fn solve(content: Vec<Vec<char>>) -> i64 {
    let mut occurences: i64 = 0;
    let row_num: usize = content.len();
    let col_num: usize = content[0].len();

    for row in 0..row_num {
        for column in 0..col_num {
            if check_horizontal(&content, row, column, Some(false)) {
                occurences += 1;
            }
            if check_horizontal(&content, row, column, Some(true)) {
                occurences += 1;
            }
            if check_vertical(&content, row, column, Some(false)) {
                occurences += 1;
            }
            if check_vertical(&content, row, column, Some(true)) {
                occurences += 1;
            }
            if check_diagonal_1_3(&content, row, column, Some(false)) {
                occurences += 1;
            }
            if check_diagonal_1_3(&content, row, column, Some(true)) {
                occurences += 1;
            }
            if check_diagonal_2_4(&content, row, column, Some(false)) {
                occurences += 1;
            }
            if check_diagonal_2_4(&content, row, column, Some(true)) {
                occurences += 1;
            }
        }
    }

    occurences
}

pub fn day4a() {
    let content = read_file("inputs/day4.txt".to_string());
    let content_matrix: Vec<Vec<char>> =
        content.lines().map(|line| line.chars().collect()).collect();

    //println!("Content: {:?}", content);
    //println!("Content_matrix: {:?}", content_matrix);

    let appearences = solve(content_matrix);

    println!("Xmas: {}", appearences);
}

#[cfg(test)]
mod tests {
    use crate::{day4::solve, utils::read_file};
    #[test]
    fn test_working() {
        let content = read_file("inputs/day4ex.txt".to_string());
        let content_matrix: Vec<Vec<char>> =
            content.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(solve(content_matrix), 18);
    }
}

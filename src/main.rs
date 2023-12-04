use std::fs;

const SYMBOLS: &str = "*?!<>{}[]()/\\|@#$%^&,;'\"`=+-_";

#[allow(dead_code)]
fn display_matrix(matrix: &[Vec<char>]) {
    for line in matrix {
        println!("{}", line.iter().collect::<String>());
    }
}

fn is_symbol(c: char) -> bool {
    SYMBOLS.contains(c)
}

fn get_input(path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines: Vec<String> = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
        .iter()
        .map(|line| -> Vec<char> { line.chars().collect() })
        .collect()
}

fn fetch_parts(matrix: &[Vec<char>]) -> u64 {
    let mut sum = 0;

    for (i, line) in matrix.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if is_symbol(*c) {
                // check surrounding positions for numbers
                let adjacent_numbers = fetch_adjacent_numbers(matrix, i, j);
                adjacent_numbers.iter().for_each(|num| sum += *num);
            }
        }
    }
    sum
}

fn fetch_adjacent_numbers(matrix: &[Vec<char>], i: usize, j: usize) -> Vec<u64> {
    let can_decrease_row = |i: usize| -> bool { i > 0 };
    let can_decrease_col = |j: usize| -> bool { j > 0 };

    let can_increase_row = |i: usize| -> bool { i < matrix.len() };
    let can_increase_col = |j: usize| -> bool { j < matrix[i].len() };

    let mut adjacent_numbers = vec![];

    let mut has_found_top_middle = false;
    // check top
    if can_decrease_row(i) {
        let top = matrix[i - 1][j];
        if top.is_numeric() {
            let num = fetch_number(matrix, i - 1, j);
            adjacent_numbers.push(num);
            has_found_top_middle = true;
        }
    }
    // check top-left
    if can_decrease_row(i) && can_decrease_col(j) {
        let top_left = matrix[i - 1][j - 1];
        if top_left.is_numeric() && !has_found_top_middle {
            let num = fetch_number(matrix, i - 1, j - 1);
            adjacent_numbers.push(num);
        }
    }
    // check top-right
    if can_decrease_row(i) && can_increase_col(j) {
        let top_right = matrix[i - 1][j + 1];
        if top_right.is_numeric() && !has_found_top_middle {
            let num = fetch_number(matrix, i - 1, j + 1);
            adjacent_numbers.push(num);
        }
    }

    // check left
    if can_decrease_col(j) {
        let left = matrix[i][j - 1];
        if left.is_numeric() {
            let num = fetch_number(matrix, i, j - 1);
            adjacent_numbers.push(num);
        }
    }

    // check right
    if can_increase_col(j) {
        let right = matrix[i][j + 1];
        if right.is_numeric() {
            let num = fetch_number(matrix, i, j + 1);
            adjacent_numbers.push(num);
        }
    }

    let mut has_found_bot_middle = false;
    // check bottom
    if can_increase_row(i) {
        let bot = matrix[i + 1][j];
        if bot.is_numeric() {
            let num = fetch_number(matrix, i + 1, j);
            adjacent_numbers.push(num);
            has_found_bot_middle = true;
        }
    }
    // check bottom-left
    if can_increase_row(i) && can_decrease_col(j) {
        let bot_left = matrix[i + 1][j - 1];
        if bot_left.is_numeric() && !has_found_bot_middle {
            let num = fetch_number(matrix, i + 1, j - 1);
            adjacent_numbers.push(num);
        }
    }
    // check bottom-right
    if can_increase_row(i) && can_increase_col(j) {
        let bot_right = matrix[i + 1][j + 1];
        if bot_right.is_numeric() && !has_found_bot_middle {
            let num = fetch_number(matrix, i + 1, j + 1);
            adjacent_numbers.push(num);
        }
    }

    adjacent_numbers
}

fn fetch_number(matrix: &[Vec<char>], x: usize, y: usize) -> u64 {
    let can_decrease =
        |y: usize| -> bool { y > 0 && y < matrix[x].len() - 1 && matrix[x][y - 1].is_numeric() };
    let can_increase =
        |y: usize| -> bool { y > 0 && y < matrix[x].len() - 1 && matrix[x][y + 1].is_numeric() };

    let mut start_y = y;
    let mut end_y = y;

    while can_decrease(start_y) {
        start_y -= 1;
    }

    while can_increase(end_y) {
        end_y += 1;
    }

    let slice: String = matrix[x][start_y..=end_y].iter().collect();
    slice.parse::<u64>().unwrap_or(0)
}

fn main() {
    let path = "res/data.txt";
    // let path = "res/data_light.txt";
    let matrix = get_input(path);
    // display_matrix(&matrix);
    println!("Sum of parts: {}", fetch_parts(&matrix));
}

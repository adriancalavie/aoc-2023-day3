use std::fs;

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

fn display_matrix(matrix: &Vec<Vec<char>>) {
    for line in matrix {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let path = "res/data.txt";
    // let path = "res/data_light.txt";
    let matrix = get_input(path);
    display_matrix(&matrix);
}

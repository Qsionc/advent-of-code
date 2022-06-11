use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_line_to_int(line: String) -> i32 {
    line.trim().parse::<i32>().expect("Error parsing int")
}

fn main() {
    let input_file_name = "input_day1_part1.txt";
    let mut relative_path = "resources/".to_owned();
    relative_path.push_str(input_file_name);
    if let Ok(path) = env::current_dir() {
        let path = path.join(Path::new(&relative_path));
        let file = File::open(path).expect("File does not exist");
        let mut reader = BufReader::new(file);

        let mut first_line = String::new();
        reader.read_line(&mut first_line).expect("Error reading first line.");
        let (mut previous, mut result): (i32, u32) = (parse_line_to_int(first_line), 0_u32);
        let mut current;
        for line in reader.lines() {
            current = parse_line_to_int(line.expect("Error reading line."));
            if current > previous {
                result += 1;
            }
            previous = current;
        }
        println!("Result: {}", result);
    }
}

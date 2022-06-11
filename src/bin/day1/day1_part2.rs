use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(relative_path: &str) -> File {
    let current_work_dir = std::env::current_dir().unwrap();
    File::open(current_work_dir.join(Path::new(&relative_path))).expect("File not found.")
}

fn construct_buf_reader(file: File) -> BufReader<File> {
    BufReader::new(file)
}

fn read_line(file: &mut BufReader<File>) -> String {
    let mut result = String::new();
    file.read_line(&mut result).expect("Error reading line");
    return result;
}

fn get_int_from_line(line: String) -> i32 {
    line.trim().parse::<i32>().expect("Error parsing int from line")
}

fn get_measurement_window(previous: &(i32, i32, i32), reading: String) -> (i32, i32, i32) {
    (previous.1, previous.2, get_int_from_line(reading))
}

fn sun_measurement(m: &(i32, i32, i32)) -> i32 {
    m.0 + m.1 + m.2
}

fn main() {
    let mut reader = construct_buf_reader(open_file("resources/input_day1_part1.txt"));
    let mut prev_window: (i32, i32, i32) = (
        get_int_from_line(read_line(&mut reader)),
        get_int_from_line(read_line(&mut reader)),
        get_int_from_line(read_line(&mut reader)),
        );
    let mut result = 0_u32;
    for line in reader.lines() {
        let curr_window = get_measurement_window(&prev_window, line.unwrap());
        if sun_measurement(&curr_window) > sun_measurement(&prev_window) {
            result += 1;
        }
        prev_window = curr_window;
    }
    println!("Result: {}", result);
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn open_file(relative_path: &str) -> File {
    let current_work_dir = std::env::current_dir().unwrap();
    File::open(current_work_dir.join(Path::new(&relative_path))).expect("File not found.")
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    fn new(dir: &str) -> Self {
        match dir.trim() {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => {
                panic!("Unknown direction:{:?}", dir);
            }
        }
    }
}

#[derive(Debug)]
struct Line {
    direction: Direction,
    value: i32,
}

impl Line {
    fn new(direction: Direction, value: i32) -> Self {
        Self { direction, value }
    }
}

fn construct_buf_reader(file: File) -> BufReader<File> {
    BufReader::new(file)
}

fn get_int(string: &str) -> i32 {
    string
        .trim()
        .parse::<i32>()
        .expect(&("Could not parse int from ".to_owned() + string))
}

fn get_line_from_reader(reader: &mut BufReader<File>) -> Option<Line> {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => {
            let line = line.split(' ').collect::<Vec<_>>();
            Some(Line::new(Direction::new(line[0].trim()), get_int(&line[1])))
        }
        Err(e) => {
            eprintln!("Error reading file:\n{:?}\n", e);
            None
        }
    }
}

fn main() {
    let (mut depth, mut horiz, mut aim) = (0, 0, 0);
    let mut reader = construct_buf_reader(open_file("resources/input_day2.txt"));
    while let Some(line) = get_line_from_reader(&mut reader) {
        match line.direction {
            Direction::Forward => {
                horiz += line.value;
                depth += line.value * aim;
            }
            Direction::Down => aim += line.value,
            Direction::Up => {
                if depth > 0 {
                    aim -= line.value
                }
            }
        }
    }
    println!("{}", depth * horiz);
}

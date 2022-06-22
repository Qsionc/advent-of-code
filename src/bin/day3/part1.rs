use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn analyze_line(vec: &mut Vec<(u32, u32)>, line: &str) {
    if vec.is_empty() {
        vec.resize(line.len(), (0, 0));
    }
    for (index, letter) in line.char_indices() {
        match letter {
            '0' => vec[index].0 += 1,
            '1' => vec[index].1 += 1,
            _ => unreachable!("Letter outside of (0, 1) list. Not a bit vector!"),
        }
    }
}

fn open_file(relative_path: &str) -> File {
    let current_work_dir = std::env::current_dir().unwrap();
    File::open(current_work_dir.join(Path::new(&relative_path))).expect("File not found.")
}

fn main() {
    let reader = BufReader::new(open_file("resources/input_day3.txt"));
    let mut analyzer = Vec::new();

    for line in reader.lines().flatten() {
        analyze_line(&mut analyzer, &line);
    }

    let mut gamma = 0_u32;
    let mut epsion = 0_u32;

    for (index, &stat) in analyzer.iter().rev().enumerate() {
        match stat.0 > stat.1 {
            true => epsion += 1 << index,
            false => gamma += 1 << index,
        }
    }

    println!("{}", gamma * epsion);
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn open_file(relative_path: &str) -> File {
    let current_work_dir = std::env::current_dir().unwrap();
    File::open(current_work_dir.join(Path::new(&relative_path))).expect("File not found.")
}

fn filter_vector_of_slices_by_bit(vec: Vec<&String>, index: usize, letter: u8) -> Vec<&String> {
    vec.into_iter()
        .filter(|k| k.as_bytes()[index] == letter)
        .collect()
}

fn analyze_vector_of_refs(vec: &Vec<&String>, index: usize) -> (i32, i32) {
    let mut result = (0, 0);
    for string in vec {
        if string.as_bytes()[index] == b'0' {
            result.0 += 1;
            continue;
        }
        result.1 += 1;
    }
    result
}

fn main() {
    let reader = BufReader::new(open_file("resources/input_day3.txt"));
    let mut bit_vectors = Vec::new();

    for line in reader.lines().flatten() {
        bit_vectors.push(line.clone());
    }

    let bit_vector_size = bit_vectors[0].len();

    let mut bit_vector_slice_life: Vec<&String> = bit_vectors.iter().collect();
    let mut bit_vector_slice_oxygen: Vec<&String> = bit_vectors.iter().collect();

    for index in 0..bit_vector_size {
        if bit_vector_slice_life.len() > 1 {
            let life_analyze = analyze_vector_of_refs(&bit_vector_slice_life, index);
            match life_analyze.1 >= life_analyze.0 {
                true => {
                    bit_vector_slice_life =
                        filter_vector_of_slices_by_bit(bit_vector_slice_life, index, b'1')
                }

                false => {
                    bit_vector_slice_life =
                        filter_vector_of_slices_by_bit(bit_vector_slice_life, index, b'0')
                }
            }
        }
        if bit_vector_slice_oxygen.len() > 1 {
            let oxygen_analyze = analyze_vector_of_refs(&bit_vector_slice_oxygen, index);
            match oxygen_analyze.0 <= oxygen_analyze.1 {
                true => {
                    bit_vector_slice_oxygen =
                        filter_vector_of_slices_by_bit(bit_vector_slice_oxygen, index, b'0');
                }
                false => {
                    bit_vector_slice_oxygen =
                        filter_vector_of_slices_by_bit(bit_vector_slice_oxygen, index, b'1');
                }
            }
        }
    }

    println!(
        "life sup: {} oxygen: {}",
        bit_vector_slice_life[0], bit_vector_slice_oxygen[0]
    );
    println!(
        "result: {}",
        u32::from_str_radix(bit_vector_slice_life[0], 2).unwrap()
            * u32::from_str_radix(bit_vector_slice_oxygen[0], 2).unwrap()
    );
}

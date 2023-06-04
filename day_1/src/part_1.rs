use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    if let Ok(lines) = read_lines(input_filename) {
        let mut capacity = 0;
        let mut best_capacity = 0;

        for line in lines {
            if let Ok(calories) = line {
                if calories.len() == 0 {
                    if best_capacity < capacity {
                        best_capacity = capacity;
                    }
                    capacity = 0;
                } else {
                    capacity += calories.parse::<i32>().unwrap();
                }
            }
        }
        println!("{}", best_capacity);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    if let Ok(lines) = read_lines(input_filename) {
        let mut capacity = 0;
        let mut capacities: Vec<i32> = vec![];

        for line in lines {
            if let Ok(calories) = line {
                if calories.len() == 0 {
                    capacities.push(capacity);
                    capacity = 0;
                } else {
                    capacity += calories.parse::<i32>().unwrap();
                }
            }
        }
        let mut heap_o_calories = BinaryHeap::from(capacities);
        let best_three = [heap_o_calories.pop().unwrap_or(-1), heap_o_calories.pop().unwrap_or(-1), heap_o_calories.pop().unwrap_or(-1)];
        println!("{:?} {}", best_three, best_three.iter().sum::<i32>());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

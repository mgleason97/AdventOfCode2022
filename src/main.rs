use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let input = get_file_content("input/day1.in");
    println!("Max calories: {}", sum_of_max_calories(&input, 3));
}

fn get_file_content(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .expect("Could not find input file for that day")
}

fn sum_of_max_calories(input: &String, num_elves: u32) -> u32 {
    let mut running_count = 0;
    let mut heap = BinaryHeap::new();

    for line in input.lines()
    {
        if line.len() > 0 {
            running_count += line.parse::<u32>().unwrap();
        } else {
            heap.push(running_count);
            running_count = 0;
        }
    }

    let mut total_calories: u32 = 0;
    for _ in 0..num_elves {
        total_calories += heap.pop().unwrap();
    }

    total_calories
}
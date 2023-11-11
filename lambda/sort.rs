use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter numbers separated by spaces: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i64> = input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    let sorted_numbers = sort_numbers(&numbers);
    println!("Sorted Numbers: {:?}", sorted_numbers);
}

fn sort_numbers(numbers: &[i64]) -> Vec<i64> {
    let mut numbers = numbers.to_vec();
    numbers.sort();
    numbers
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day4 input-filename");

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");
    for line in input.lines() {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_1() {
        assert_eq!(1 + 1, 2);
    }
}

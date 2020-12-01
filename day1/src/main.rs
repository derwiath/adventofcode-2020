use std::env;
use std::fs;

fn find_terms(terms: &[i32], sum: i32) -> Option<(i32, i32)> {
    terms.iter().enumerate().find_map(|(i1, t1)| {
        let needle: i32 = sum - t1;
        if let Some(i2) = terms.iter().skip(i1).position(|t2| *t2 == needle) {
            Some((*t1, terms[i1 + i2]))
        } else {
            None
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day1 input-filename");

    println!("Reading terms from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let terms: Vec<i32> = input
        .lines()
        .map(|s| s.parse::<i32>().expect("Failed to parse i32"))
        .collect();
    match find_terms(&terms[..], 2020) {
        Some((t1, t2)) => {
            println!("{} + {} = {}", t1, t2, t1 + t2);
            println!("{} * {} = {}", t1, t2, t1 * t2);
        }
        None => {
            println!("Failed to find terms that add up to 2020");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TERMS: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_example() {
        assert_eq!(find_terms(&EXAMPLE_TERMS, 2020), Some((1721, 299)));
    }
}

use std::env;
use std::fs;

fn find_terms2(terms: &[i32], sum: i32) -> Option<(i32, i32)> {
    terms.iter().enumerate().find_map(|(i1, t1)| {
        let needle: i32 = sum - t1;
        if let Some(i2) = terms.iter().skip(i1).position(|t2| *t2 == needle) {
            Some((*t1, terms[i1 + i2]))
        } else {
            None
        }
    })
}

fn find_terms3(terms: &[i32], sum: i32) -> Option<(i32, i32, i32)> {
    terms.iter().enumerate().find_map(|(i1, t1)| {
        if let Some((t2, t3)) = find_terms2(&terms[i1 + 1..], sum - t1) {
            Some((*t1, t2, t3))
        } else {
            None
        }
    })
}

fn solve_part1(terms: &[i32], sum: i32) {
    match find_terms2(&terms[..], sum) {
        Some((t1, t2)) => {
            println!("{} + {} = {}", t1, t2, t1 + t2);
            println!("{} * {} = {}", t1, t2, t1 * t2);
        }
        None => {
            println!("Failed to find terms that add up to {}", sum);
        }
    }
}

fn solve_part2(terms: &[i32], sum: i32) {
    match find_terms3(&terms[..], sum) {
        Some((t1, t2, t3)) => {
            println!("{} + {} + {} = {}", t1, t2, t3, t1 + t2 + t3);
            println!("{} * {} * {} = {}", t1, t2, t3, t1 * t2 * t3);
        }
        None => {
            println!("Failed to find terms that add up to {}", sum);
        }
    }
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
    solve_part1(&terms[..], 2020);
    solve_part2(&terms[..], 2020);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TERMS: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_example_two_terms() {
        assert_eq!(find_terms2(&EXAMPLE_TERMS, 2020), Some((1721, 299)));
    }

    #[test]
    fn test_example_three_terms() {
        assert_eq!(find_terms3(&EXAMPLE_TERMS, 2020), Some((979, 366, 675)));
    }
}

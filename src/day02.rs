use std::collections::HashMap;

/// Compute checksum for the given list of boxes.
///
/// # Examples
///
/// ```
/// use aoc18::day02::checksum;
///
/// assert_eq!(12, checksum("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"));
/// ```
pub fn checksum(input: &str) -> u64 {
    let lines: Vec<&str> = input
        .split("\n")
        .map(|s| s.trim())
        .collect();
    let mut twos = 0;
    let mut threes = 0;
    for line in lines {
        let mut freq = HashMap::new();
        for c in line.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        let mut has_two = false;
        let mut has_three = false;
        for v in freq.values() {
            if !has_two && *v == 2  {
                has_two = true;
                twos += 1;
            }
            if !has_three && *v == 3 {
                has_three = true;
                threes += 1;
            }
        }
    }
    twos * threes
}

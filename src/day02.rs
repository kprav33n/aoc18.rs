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
    let lines: Vec<&str> = input.split('\n').map(|s| s.trim()).collect();
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
            if !has_two && *v == 2 {
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

/// Compute letters that are common between two correct box IDs.
///
/// # Examples
///
/// ```
/// use aoc18::day02::common_letters;
///
/// assert_eq!("fgij", common_letters("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"));
/// ```
pub fn common_letters(input: &str) -> String {
    let lines: Vec<&str> = input.split('\n').map(|s| s.trim()).collect();

    for i in 0..(lines.len() - 1) {
        for j in (i + 1)..lines.len() {
            if let Some(x) = singleton_diff_index(lines[i], lines[j]) {
                return lines[i]
                    .chars()
                    .enumerate()
                    .filter(|(p, _)| *p != x)
                    .map(|(_, c)| c)
                    .collect::<String>();
            } else {
                continue;
            }
        }
    }
    String::new()
}

fn singleton_diff_index(one: &str, other: &str) -> Option<usize> {
    let mut diffs = Vec::new();
    for (i, (x, y)) in one.chars().zip(other.chars()).enumerate() {
        if x != y {
            diffs.push(i);
        }
    }
    if diffs.len() == 1 {
        Some(diffs[0])
    } else {
        None
    }
}

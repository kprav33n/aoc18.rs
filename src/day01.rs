use std::collections::HashSet;

/// Compute resulting frequency by summing a list of deltas that are seperated
/// by a line break.
///
/// # Examples
///
/// ```
/// use aoc18::day01::resulting_frequency;
///
/// assert_eq!(8, resulting_frequency("+8"));
/// assert_eq!(-3, resulting_frequency("-3"));
/// assert_eq!(3, resulting_frequency("+1\n+1\n+1"));
/// assert_eq!(0, resulting_frequency("+1\n+1\n-2"));
/// assert_eq!(-6, resulting_frequency("-1\n-2\n-3"));
/// ```
pub fn resulting_frequency(input: &str) -> i64 {
    input
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .sum()
}

/// Compute the frequency that's reached twice from a list of deltas seperated
/// by a line break.
///
/// # Examples
///
/// ```
/// use aoc18::day01::resulting_frequency_twice;
///
/// assert_eq!(0, resulting_frequency_twice("+1\n-1"));
/// assert_eq!(10, resulting_frequency_twice("+3\n+3\n+4\n-2\n-4"));
/// assert_eq!(5, resulting_frequency_twice("-6\n+3\n+8\n+5\n-6"));
/// assert_eq!(14, resulting_frequency_twice("+7\n+7\n-2\n-7\n-4"));
/// ```
pub fn resulting_frequency_twice(input: &str) -> i64 {
    let mut frequencies = HashSet::new();
    let deltas: Vec<i64> = input
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let mut result = 0;
    frequencies.insert(result);
    for delta in deltas.iter().cycle() {
        result += delta;
        if !frequencies.contains(&result) {
            frequencies.insert(result);
        } else {
            break;
        }
    }
    result
}

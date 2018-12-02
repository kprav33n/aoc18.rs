/// Compute resulting frequency from a list of deltas seperated by a line break.
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
/// assert_eq!(-5, resulting_frequency("-1\n-2\n-3"));
/// ```
pub fn resulting_frequency(input: &str) -> i64 {
    input.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.parse::<i64>().unwrap()).fold(0, |acc, x| acc + x)
}

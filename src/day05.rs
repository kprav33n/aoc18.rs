/// Find the number of remaining units after reaction in the given sequence of
/// units.
///
/// # Examples
///
/// ```
/// use aoc18::day05::num_remaining_units;
///
/// assert_eq!(10, num_remaining_units("dabAcCaCBAcCcaDA"));
/// ```
pub fn num_remaining_units(input: &str) -> usize {
    let chars = input.trim().chars().collect::<Vec<char>>();
    let mut left: i64 = 0;
    let mut right: i64 = 1;
    let mut unburned_left: Vec<i64> = Vec::new();
    let mut num_burned = 0;
    while right < chars.len() as i64 {
        if should_burn(chars[left as usize], chars[right as usize]) {
            num_burned += 2;
            right += 1;
            if unburned_left.is_empty() {
                left = right;
                unburned_left.push(left);
                right += 1;
            } else {
                left = unburned_left.pop().unwrap();
            }
        } else {
            unburned_left.push(left);
            left = right;
            right += 1;
        }
    }
    chars.len() - num_burned
}

/// Find the smallest number of remaining units after reaction in the given
/// sequence of units.
///
/// # Examples
///
/// ```
/// use aoc18::day05::smallest_num_remaining_units;
///
/// assert_eq!(4, smallest_num_remaining_units("dabAcCaCBAcCcaDA"));
/// ```
pub fn smallest_num_remaining_units(input: &str) -> usize {
    let mut min = std::usize::MAX;
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let n = num_remaining_units(
            &input
                .chars()
                .filter(|x| !x.eq_ignore_ascii_case(&c))
                .collect::<String>(),
        );
        if min > n {
            min = n;
        }
    }
    min
}

fn should_burn(x: char, y: char) -> bool {
    (x as i32 - y as i32).abs() == 32
}

#[test]
fn test_should_burn() {
    assert!(should_burn('a', 'A'));
    assert!(should_burn('M', 'm'));
    assert!(!should_burn('N', 'N'));
    assert!(!should_burn('z', 'z'));
    assert!(!should_burn('a', 'z'));
}

use std::char;

/// Find the score of ten recipes after given number of recipes.
///
/// # Examples
///
/// ```
/// use aoc18::day14::{score_after};
///
/// assert_eq!("5158916779", score_after("9"));
/// assert_eq!("0124515891", score_after("5"));
/// assert_eq!("9251071085", score_after("18"));
/// assert_eq!("5941429882", score_after("2018"));
/// ```
pub fn score_after(input: &str) -> String {
    let mut recipes = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;
    let n: usize = input.trim().parse().unwrap();
    while recipes.len() < n + 10 {
        let v1 = recipes[e1];
        let v2 = recipes[e2];
        let s = v1 + v2;
        let units = s % 10;
        let tens = s / 10;
        if tens > 0 {
            recipes.push(tens);
        }
        recipes.push(units);
        e1 = (e1 + v1 + 1) % recipes.len();
        e2 = (e2 + v2 + 1) % recipes.len();
    }
    recipes
        .iter()
        .skip(n)
        .take(10)
        .map(|i| char::from_digit(*i as u32, 10).unwrap())
        .collect()
}

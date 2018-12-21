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

/// Find the number of recipes appear on the scoreboard to the left of the given
/// sequence.
///
/// # Examples
///
/// ```
/// use aoc18::day14::{num_recipes_before};
///
/// assert_eq!(9, num_recipes_before("51589"));
/// assert_eq!(5, num_recipes_before("01245"));
/// assert_eq!(18, num_recipes_before("92510"));
/// assert_eq!(2018, num_recipes_before("59414"));
/// ```
pub fn num_recipes_before(input: &str) -> usize {
    let mut recipes: Vec<u8> = Vec::with_capacity(50_000_000);
    recipes.push(3);
    recipes.push(7);
    let mut e1 = 0;
    let mut e2 = 1;
    let suffix: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let mut found = 0;
    while found == 0 {
        let v1 = recipes[e1];
        let v2 = recipes[e2];
        let s = v1 + v2;
        if s > 9 {
            recipes.push(1);
            recipes.push(s % 10);
        } else {
            recipes.push(s);
        }
        e1 = (e1 + v1 as usize + 1) % recipes.len();
        e2 = (e2 + v2 as usize + 1) % recipes.len();

        found = 2;
        for (x, y) in suffix.iter().rev().zip(recipes.iter().rev()) {
            if x != y {
                found -= 1;
                for (x, y) in suffix.iter().rev().zip(recipes.iter().rev().skip(1)) {
                    if x != y {
                        found -= 1;
                        break;
                    }
                }
                break;
            }
        }
    }
    recipes.len() - suffix.len() - (2 - found)
}

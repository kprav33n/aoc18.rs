use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

/// Find the size of the largest finite area based on given list of coordinates.
///
/// # Examples
///
/// ```
/// use aoc18::day06::largest_area;
///
/// assert_eq!(17, largest_area("1, 1
/// 1, 6
/// 8, 3
/// 3, 4
/// 5, 5
/// 8, 9"));
/// ```
pub fn largest_area(input: &str) -> usize {
    let coordinates = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| Point::from_str(s).unwrap())
        .collect::<Vec<Point>>();

    const BOUND: usize = 500;
    let mut plane = [[(0 as i64, std::i64::MAX); BOUND]; BOUND];
    for (i, r) in plane.iter_mut().enumerate() {
        for (j, p) in r.iter_mut().enumerate() {
            for (n, c) in coordinates.iter().enumerate() {
                let d = c.manhattan_distance(&Point {
                    x: i as i64,
                    y: j as i64,
                });
                if d < p.1 {
                    *p = (n as i64, d);
                } else if d == p.1 && n as i64 != p.0 {
                    *p = (-1, d);
                }
            }
        }
    }

    let mut outliers = HashSet::new();
    for i in 0..BOUND {
        outliers.insert(plane[0][i].0);
        outliers.insert(plane[i][0].0);
        outliers.insert(plane[BOUND - 1][i].0);
        outliers.insert(plane[i][BOUND - 1].0);
    }

    let mut counter = HashMap::new();
    for r in plane.iter() {
        for p in r.iter() {
            if !outliers.contains(&p.0) {
                *counter.entry(p.0).or_insert(0) += 1;
            }
        }
    }

    counter
        .iter()
        .fold(0, |max, (_, &v)| if max < v { v } else { max })
}

/// Find the size of the largest finite area based on given list of coordinates.
///
/// # Examples
///
/// ```
/// use aoc18::day06::safe_area;
///
/// assert_eq!(16, safe_area("1, 1
/// 1, 6
/// 8, 3
/// 3, 4
/// 5, 5
/// 8, 9", 32));
/// ```
pub fn safe_area(input: &str, limit: i64) -> usize {
    let coordinates = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| Point::from_str(s).unwrap())
        .collect::<Vec<Point>>();

    const BOUND: usize = 500;
    let mut plane = [[0; BOUND]; BOUND];
    for (i, r) in plane.iter_mut().enumerate() {
        for (j, c) in r.iter_mut().enumerate() {
            let mut sum = 0;
            for c in coordinates.iter() {
                let d = c.manhattan_distance(&Point {
                    x: i as i64,
                    y: j as i64,
                });
                sum += d;
            }
            *c = sum;
        }
    }

    let mut result = 0;
    for r in plane.iter() {
        for c in r.iter() {
            if *c < limit {
                result += 1;
            }
        }
    }
    result
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(", ").collect();

        let x_fromstr = coords[0].parse::<i64>()?;
        let y_fromstr = coords[1].parse::<i64>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

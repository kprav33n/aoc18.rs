use std::fmt;

/// Find the winning Elf's score.
///
/// # Examples
///
/// ```
/// use aoc18::day11::{Point, largest_powered_cell};
///
/// assert_eq!(Point{x: 33, y: 45}, largest_powered_cell("18"));
/// assert_eq!(Point{x:21, y: 61}, largest_powered_cell("42"));
/// ```
pub fn largest_powered_cell(serial: &str) -> Point {
    let s = serial.trim().parse::<i64>().unwrap();
    const SIZE: usize = 300;
    let mut grid = [[0 as i64; SIZE]; SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            let rack_id = (x as i64 + 1) + 10;
            // TODO: Simplify to a single expression.
            let mut power_level = rack_id * (y as i64 + 1);
            power_level += s;
            power_level *= rack_id;
            power_level = (power_level % 1000) / 100;
            power_level -= 5;
            grid[x][y] = power_level;
        }
    }

    let mut max_power_level = 0;
    let mut max_point = Point { x: 0, y: 0 };
    for x in 0..SIZE - 3 {
        for y in 0..SIZE - 3 {
            let mut grid_power = 0;
            for i in 0..3 {
                for j in 0..3 {
                    grid_power += grid[x + i][y + j];
                }
            }
            if max_power_level < grid_power {
                max_power_level = grid_power;
                max_point = Point { x: x + 1, y: y + 1 };
            }
        }
    }
    max_point
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

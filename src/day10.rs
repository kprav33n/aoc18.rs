use regex::{Error, Regex};
use std::str::FromStr;

/// Find the message, and number of seconds taken to reach that.
///
/// # Examples
///
/// ```
/// use aoc18::day10::message;
///
/// let (m, n) = message("position=< 9,  1> velocity=< 0,  2>
/// position=< 7,  0> velocity=<-1,  0>
/// position=< 3, -2> velocity=<-1,  1>
/// position=< 6, 10> velocity=<-2, -1>
/// position=< 2, -4> velocity=< 2,  2>
/// position=<-6, 10> velocity=< 2, -2>
/// position=< 1,  8> velocity=< 1, -1>
/// position=< 1,  7> velocity=< 1,  0>
/// position=<-3, 11> velocity=< 1, -2>
/// position=< 7,  6> velocity=<-1, -1>
/// position=<-2,  3> velocity=< 1,  0>
/// position=<-4,  3> velocity=< 2,  0>
/// position=<10, -3> velocity=<-1,  1>
/// position=< 5, 11> velocity=< 1, -2>
/// position=< 4,  7> velocity=< 0, -1>
/// position=< 8, -2> velocity=< 0,  1>
/// position=<15,  0> velocity=<-2,  0>
/// position=< 1,  6> velocity=< 1,  0>
/// position=< 8,  9> velocity=< 0, -1>
/// position=< 3,  3> velocity=<-1,  1>
/// position=< 0,  5> velocity=< 0, -1>
/// position=<-2,  2> velocity=< 2,  0>
/// position=< 5, -2> velocity=< 1,  2>
/// position=< 1,  4> velocity=< 2,  1>
/// position=<-2,  7> velocity=< 2, -2>
/// position=< 3,  6> velocity=<-1, -1>
/// position=< 5,  0> velocity=< 1,  0>
/// position=<-6,  0> velocity=< 2,  0>
/// position=< 5,  9> velocity=< 1, -2>
/// position=<14,  7> velocity=<-2,  0>
/// position=<-3,  6> velocity=< 2, -1>");
/// assert_eq!(3, n);
/// ```
pub fn message(input: &str) -> (String, usize) {
    let mut entries = input
        .trim()
        .split('\n')
        .map(Entry::from_str)
        .map(Result::unwrap)
        .collect::<Vec<Entry>>();
    let mut last_entries = vec![Entry {
        location: Point { x: 0, y: 0 },
        velocity: Point { x: 0, y: 0 },
    }];
    let mut last_width = std::i64::MAX;
    let mut last_height = std::i64::MAX;
    let mut last_min_x = 0;
    let mut last_min_y = 0;
    let mut last_max_x = 0;
    let mut last_max_y = 0;
    for i in 0..100_000 {
        entries = entries.iter().map(Entry::next).collect();
        let (min_x, min_y, max_x, max_y) = entries_bounds(&entries);
        let width = max_x - min_x;
        let height = max_y - min_y;
        if last_width - width < 0 && last_height - height < 0 {
            return (
                plot_entries(
                    &last_entries,
                    last_min_x,
                    last_min_y,
                    last_max_x,
                    last_max_y,
                ),
                i,
            );
        } else {
            last_width = width;
            last_height = height;
            last_min_x = min_x;
            last_min_y = min_y;
            last_max_x = max_x;
            last_max_y = max_y;
            last_entries = entries.to_vec();
        }
    }
    (String::from(""), 0)
}

fn entries_bounds(entries: &[Entry]) -> (i64, i64, i64, i64) {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;
    for entry in entries {
        min_x = std::cmp::min(min_x, entry.location.x);
        min_y = std::cmp::min(min_y, entry.location.y);
        max_x = std::cmp::max(max_x, entry.location.x);
        max_y = std::cmp::max(max_y, entry.location.y);
    }
    (min_x, min_y, max_x, max_y)
}

fn plot_entries(entries: &[Entry], min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> String {
    let mut bitmap: Vec<Vec<char>> = (min_y..=max_y)
        .map(|_| (min_x..=max_x).map(|_| '.').collect())
        .collect();
    for entry in entries {
        let l = &entry.location;
        bitmap[(l.y - min_y) as usize][(l.x - min_x) as usize] = '#';
    }
    bitmap
        .iter()
        .map(|v| v.into_iter().collect::<String>() + "\n")
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    location: Point,
    velocity: Point,
}

impl Entry {
    fn next(&self) -> Entry {
        Entry {
            location: {
                Point {
                    x: self.location.x + self.velocity.x,
                    y: self.location.y + self.velocity.y,
                }
            },
            velocity: {
                Point {
                    x: self.velocity.x,
                    y: self.velocity.y,
                }
            },
        }
    }
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"position=<(?P<x1>[ -]?[[:digit:]]+), (?P<y1>[ -]?[[:digit:]]+)> velocity=<(?P<x2>[ -]?[[:digit:]]+), (?P<y2>[ -]?[[:digit:]]+)>").unwrap();
        let caps = re.captures(s.trim()).unwrap();
        // NOTE: I don't handle parsing errors correctly here.
        Ok(Entry {
            location: Point {
                x: caps["x1"].trim().parse().unwrap(),
                y: caps["y1"].trim().parse().unwrap(),
            },
            velocity: Point {
                x: caps["x2"].trim().parse().unwrap(),
                y: caps["y2"].trim().parse().unwrap(),
            },
        })
    }
}

#[test]
fn test_entry_from_str() {
    assert_eq!(
        Ok(Entry {
            location: Point { x: -3, y: 6 },
            velocity: Point { x: 2, y: -1 }
        }),
        Entry::from_str("position=<-3,  6> velocity=< 2, -1>")
    );
}

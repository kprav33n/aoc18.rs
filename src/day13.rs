use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

/// Find the location of the first crash.
pub fn first_crash(input: &str) -> Point {
    let mut grids: Vec<Vec<Grid>> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|r| r.chars().map(|c| Grid::new(c)).collect())
        .collect();
    // Pad a border.
    let columns = grids[0].len();
    for r in &mut grids {
        r.insert(0, Grid::NA);
        r.push(Grid::NA);
    }
    grids.insert(0, vec![Grid::NA; columns + 2]);
    grids.push(vec![Grid::NA; columns + 2]);

    let mut carts: Vec<Cart> = input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, r)| {
            r.chars()
                .enumerate()
                .map(|(j, c)| Cart::new(c, j, i))
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter_map(|o| o)
        .collect();
    loop {
        // print_state(&grids, &carts);
        for cart in &mut carts {
            cart.next(&grids);
        }
        match duplicate(
            &carts
                .iter()
                .map(|c| Point {
                    x: c.point.x,
                    y: c.point.y,
                })
                .collect::<Vec<_>>(),
        ) {
            Some(p) => {
                return Point {
                    x: p.x - 1,
                    y: p.y - 1,
                };
            }

            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
enum Grid {
    NA,
    Horizontal,
    Vertical,
    Intersection,
    Turn,
}

impl Grid {
    fn new(ch: char) -> Self {
        match ch {
            '-' => Grid::Horizontal,
            '>' => Grid::Horizontal,
            '<' => Grid::Horizontal,
            '|' => Grid::Vertical,
            '^' => Grid::Vertical,
            'v' => Grid::Vertical,
            '+' => Grid::Intersection,
            '\\' => Grid::Turn,
            '/' => Grid::Turn,
            _ => Grid::NA,
        }
    }
}

#[derive(Debug)]
enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
enum Decision {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct Cart {
    point: Point,
    orientation: Orientation,
    decisions: VecDeque<Decision>,
}

impl Cart {
    fn new(ch: char, x: usize, y: usize) -> Option<Cart> {
        let point = Point { x: x + 1, y: y + 1 };
        let mut decisions = VecDeque::new();
        decisions.push_back(Decision::Left);
        decisions.push_back(Decision::Straight);
        decisions.push_back(Decision::Right);
        match ch {
            '>' => Some(Cart {
                point,
                orientation: Orientation::Right,
                decisions,
            }),
            '<' => Some(Cart {
                point,
                orientation: Orientation::Left,
                decisions,
            }),
            '^' => Some(Cart {
                point,
                orientation: Orientation::Up,
                decisions,
            }),
            'v' => Some(Cart {
                point,
                orientation: Orientation::Down,
                decisions,
            }),
            _ => None,
        }
    }

    fn next(&mut self, grid: &Vec<Vec<Grid>>) {
        match self.orientation {
            Orientation::Left => {
                self.point.x -= 1;
            }
            Orientation::Right => {
                self.point.x += 1;
            }
            Orientation::Up => {
                self.point.y -= 1;
            }
            Orientation::Down => {
                self.point.y += 1;
            }
        }

        match grid[self.point.y][self.point.x] {
            Grid::Turn => match self.orientation {
                Orientation::Left | Orientation::Right => {
                    if let Grid::Vertical = grid[self.point.y - 1][self.point.x] {
                        self.orientation = Orientation::Up;
                    } else if let Grid::Intersection = grid[self.point.y - 1][self.point.x] {
                        self.orientation = Orientation::Up;
                    } else if let Grid::Vertical = grid[self.point.y + 1][self.point.x] {
                        self.orientation = Orientation::Down;
                    } else if let Grid::Intersection = grid[self.point.y + 1][self.point.x] {
                        self.orientation = Orientation::Down;
                    }
                }
                Orientation::Up | Orientation::Down => {
                    if let Grid::Horizontal = grid[self.point.y][self.point.x - 1] {
                        self.orientation = Orientation::Left;
                    } else if let Grid::Intersection = grid[self.point.y][self.point.x - 1] {
                        self.orientation = Orientation::Left;
                    } else if let Grid::Horizontal = grid[self.point.y][self.point.x + 1] {
                        self.orientation = Orientation::Right;
                    } else if let Grid::Intersection = grid[self.point.y][self.point.x + 1] {
                        self.orientation = Orientation::Right;
                    }
                }
            },
            Grid::Intersection => {
                let decision = self.decisions.pop_front().unwrap();
                self.decisions.push_back(decision.clone());
                match decision {
                    Decision::Left => match self.orientation {
                        Orientation::Left => {
                            self.orientation = Orientation::Down;
                        }
                        Orientation::Right => {
                            self.orientation = Orientation::Up;
                        }
                        Orientation::Up => {
                            self.orientation = Orientation::Left;
                        }
                        Orientation::Down => {
                            self.orientation = Orientation::Right;
                        }
                    },
                    Decision::Right => match self.orientation {
                        Orientation::Left => {
                            self.orientation = Orientation::Up;
                        }
                        Orientation::Right => {
                            self.orientation = Orientation::Down;
                        }
                        Orientation::Up => {
                            self.orientation = Orientation::Right;
                        }
                        Orientation::Down => {
                            self.orientation = Orientation::Left;
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn duplicate(points: &[Point]) -> Option<Point> {
    let mut set: HashSet<Point> = HashSet::new();
    for p in points {
        if set.contains(p) {
            return Some(p.clone());
        }
        set.insert(p.clone());
    }
    return None;
}

#[allow(dead_code)]
fn print_state(grid: &Vec<Vec<Grid>>, carts: &Vec<Cart>) {
    let mut map: Vec<Vec<char>> = grid
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| match c {
                    Grid::Horizontal => '-',
                    Grid::Intersection => '+',
                    Grid::Vertical => '|',
                    Grid::Turn => '#',
                    Grid::NA => ' ',
                })
                .collect()
        })
        .collect();
    for c in carts {
        map[c.point.y][c.point.x] = match c.orientation {
            Orientation::Left => '<',
            Orientation::Right => '>',
            Orientation::Up => '^',
            Orientation::Down => 'v',
        }
    }
    for r in map {
        println!("{}", r.iter().collect::<String>());
    }
    println!("");
}

#[test]
fn test_first_crash() {
    let p = first_crash(
        "/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   
",
    );
    assert_eq!(p.x, 7);
    assert_eq!(p.y, 3);
}

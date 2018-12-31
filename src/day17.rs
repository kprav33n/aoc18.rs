use std::fmt;

/// Determine the number of tiles that the water can reach in the given range of
/// y values.
///
/// ```
/// use aoc18::day17::reservoir_reach;
/// assert_eq!(57, reservoir_reach("x=495, y=2..7
/// y=7, x=495..501
/// x=501, y=3..7
/// x=498, y=2..4
/// x=506, y=1..2
/// x=498, y=10..13
/// x=504, y=10..13
/// y=13, x=498..504"));
/// ```
pub fn reservoir_reach(input: &str) -> usize {
    let scans: Vec<Point> = input.trim().split('\n').map(parse_scan).flatten().collect();
    let mut ground = Ground::new(&scans);
    while !ground.can_end() {
        ground.next_cycle();
    }
    ground.reach()
}

/// Determine the number of stored water tiles in the given range of y values.
///
/// ```
/// use aoc18::day17::reservoir_capacity;
/// assert_eq!(29, reservoir_capacity("x=495, y=2..7
/// y=7, x=495..501
/// x=501, y=3..7
/// x=498, y=2..4
/// x=506, y=1..2
/// x=498, y=10..13
/// x=504, y=10..13
/// y=13, x=498..504"));
/// ```
pub fn reservoir_capacity(input: &str) -> usize {
    // todo: refactor part 1 and part 2.
    let scans: Vec<Point> = input.trim().split('\n').map(parse_scan).flatten().collect();
    let mut ground = Ground::new(&scans);
    while !ground.can_end() {
        ground.next_cycle();
    }
    ground.capacity()
}

#[derive(Clone, Debug)]
enum Cell {
    Sand,
    Clay,
    WaterSpring,
    WaterFlowing,
    WaterAtRest,
}

#[derive(Debug)]
struct Point(usize, usize);

fn parse_scan(input: &str) -> Vec<Point> {
    let fields = input.trim().split(", ");
    let mut xs = vec![];
    let mut ys = vec![];
    for f in fields {
        let ranges: Vec<&str> = f[2..].split("..").collect();
        let values: Vec<usize> = if ranges.len() == 1 {
            vec![ranges[0].parse().unwrap()]
        } else {
            (ranges[0].parse().unwrap()..=ranges[1].parse().unwrap()).collect()
        };
        match &f[..2] {
            "x=" => {
                xs = values;
            }
            "y=" => {
                ys = values;
            }
            _ => {
                panic!("boom");
            }
        }
    }
    let l = std::cmp::max(xs.len(), ys.len());
    xs.iter()
        .cycle()
        .take(l)
        .zip(ys.iter().cycle().take(l))
        .map(|(&x, &y)| Point(x, y))
        .collect()
}

struct Ground {
    // left: usize,
    // right: usize,
    top: usize,
    // bottom: usize,
    cells: Vec<Vec<Cell>>,
}

impl Ground {
    fn new(scans: &[Point]) -> Ground {
        let mut left = std::usize::MAX;
        let mut top = std::usize::MAX;
        let mut right = std::usize::MIN;
        let mut bottom = std::usize::MIN;
        for s in scans {
            left = std::cmp::min(left, s.0);
            top = std::cmp::min(top, s.1);
            right = std::cmp::max(right, s.0);
            bottom = std::cmp::max(bottom, s.1);
        }
        left -= 1;
        right += 1;
        bottom += 1;
        let mut cells: Vec<Vec<Cell>> = (0..=bottom)
            .map(|_| (left..=right).map(|_| Cell::Sand).collect())
            .collect();
        for s in scans {
            cells[s.1][s.0 - left] = Cell::Clay;
        }
        cells[0][500 - left] = Cell::WaterSpring;
        Ground {
            // left,
            // right,
            top,
            // bottom,
            cells,
        }
    }

    fn next_cycle(&mut self) {
        for i in 1..self.cells.len() - 1 {
            for j in 0..self.cells[i].len() {
                match self.cells[i - 1][j] {
                    Cell::WaterSpring | Cell::WaterFlowing => {
                        self.cells[i][j] = match self.cells[i][j] {
                            Cell::Sand => Cell::WaterFlowing,
                            _ => self.cells[i][j].clone(),
                        };
                    }

                    _ => {}
                }

                match (&self.cells[i][j], &self.cells[i + 1][j]) {
                    (Cell::WaterFlowing, Cell::Clay) | (Cell::WaterFlowing, Cell::WaterAtRest) => {
                        let mut current = j - 1;
                        let mut left = None;
                        loop {
                            match (&self.cells[i][current], &self.cells[i + 1][current]) {
                                (Cell::Sand, Cell::Clay)
                                | (Cell::Sand, Cell::WaterAtRest)
                                | (Cell::WaterFlowing, Cell::Clay)
                                | (Cell::WaterFlowing, Cell::WaterAtRest) => {
                                    current -= 1;
                                }

                                (Cell::Clay, Cell::Clay) | (Cell::Clay, Cell::WaterAtRest) => {
                                    left = Some(current);
                                    break;
                                }

                                _ => {
                                    break;
                                }
                            }
                        }
                        let llimit = current;

                        current = j + 1;
                        let mut right = None;
                        loop {
                            match (&self.cells[i][current], &self.cells[i + 1][current]) {
                                (Cell::Sand, Cell::Clay)
                                | (Cell::Sand, Cell::WaterAtRest)
                                | (Cell::WaterFlowing, Cell::Clay)
                                | (Cell::WaterFlowing, Cell::WaterAtRest) => {
                                    current += 1;
                                }
                                (Cell::Clay, Cell::Clay) | (Cell::Clay, Cell::WaterAtRest) => {
                                    right = Some(current);
                                    break;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                        let rlimit = current;

                        match (left, right) {
                            (Some(l), Some(r)) => {
                                for j in (l + 1)..r {
                                    self.cells[i][j] = Cell::WaterAtRest;
                                }
                            }
                            (Some(l), None) => {
                                for j in (l + 1)..=rlimit {
                                    self.cells[i][j] = Cell::WaterFlowing;
                                }
                            }
                            (None, Some(r)) => {
                                for j in llimit..r {
                                    self.cells[i][j] = Cell::WaterFlowing;
                                }
                            }
                            (None, None) => {
                                for j in llimit..=rlimit {
                                    self.cells[i][j] = Cell::WaterFlowing;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn reach(&self) -> usize {
        self.cells
            .iter()
            .skip(self.top)
            .flatten()
            .filter(|c| match c {
                Cell::WaterFlowing | Cell::WaterAtRest => true,
                _ => false,
            })
            .count()
    }

    fn capacity(&self) -> usize {
        self.cells
            .iter()
            .skip(self.top)
            .flatten()
            .filter(|c| {
                if let Cell::WaterAtRest = c {
                    true
                } else {
                    false
                }
            })
            .count()
    }

    fn can_end(&self) -> bool {
        let l = self.cells.len();
        self.cells[l - 2]
            .iter()
            .filter(|c| match c {
                Cell::WaterFlowing | Cell::WaterAtRest => true,
                _ => false,
            })
            .count()
            > 0
    }
}

impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr: String = self
            .cells
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| match c {
                        Cell::Sand => '.',
                        Cell::Clay => '#',
                        Cell::WaterSpring => '+',
                        Cell::WaterFlowing => '|',
                        Cell::WaterAtRest => '~',
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect();
        write!(f, "{}", repr)
    }
}

use pathfinding::prelude::{astar_bag, bfs};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Determine the combat outcome given the initial position.
///
/// # Examples
///
/// ```
/// use aoc18::day15::combat_outcome;
///
/// assert_eq!(27730, combat_outcome("#######
/// #.G...#
/// #...EG#
/// #.#.#G#
/// #..G#E#
/// #.....#
/// #######
/// "));
/// assert_eq!(36334, combat_outcome("#######
/// #G..#E#
/// #E#E.E#
/// #G.##.#
/// #...#E#
/// #...E.#
/// #######
/// "));
/// assert_eq!(39514, combat_outcome("#######
/// #E..EG#
/// #.#G.E#
/// #E.##E#
/// #G..#.#
/// #..E#.#
/// #######
/// "));
/// assert_eq!(27755, combat_outcome("#######
/// #E.G#.#
/// #.#G..#
/// #G.#.G#
/// #G..#.#
/// #...E.#
/// #######
/// "));
/// assert_eq!(28944, combat_outcome("#######
/// #.E...#
/// #.#..G#
/// #.###.#
/// #E#G#G#
/// #...#G#
/// #######
/// "));
/// assert_eq!(18740, combat_outcome("#########
/// #G......#
/// #.E.#...#
/// #..##..G#
/// #...##..#
/// #...#...#
/// #.G...G.#
/// #.....G.#
/// #########
/// "));
/// ```
pub fn combat_outcome(input: &str) -> usize {
    let mut game = Game::new(input, 3);
    while game.next_round() {}
    game.outcome()
}

/// Determine the rigged combat outcome given the initial position.
///
/// # Examples
///
/// ```
/// use aoc18::day15::combat_outcome2;
///
/// assert_eq!(4988, combat_outcome2("#######
/// #.G...#
/// #...EG#
/// #.#.#G#
/// #..G#E#
/// #.....#
/// #######
/// "));
/// assert_eq!(31284, combat_outcome2("#######
/// #E..EG#
/// #.#G.E#
/// #E.##E#
/// #G..#.#
/// #..E#.#
/// #######
/// "));
/// assert_eq!(3478, combat_outcome2("#######
/// #E.G#.#
/// #.#G..#
/// #G.#.G#
/// #G..#.#
/// #...E.#
/// #######
/// "));
/// assert_eq!(6474, combat_outcome2("#######
/// #.E...#
/// #.#..G#
/// #.###.#
/// #E#G#G#
/// #...#G#
/// #######
/// "));
/// assert_eq!(1140, combat_outcome2("#########
/// #G......#
/// #.E.#...#
/// #..##..G#
/// #...##..#
/// #...#...#
/// #.G...G.#
/// #.....G.#
/// #########
/// "));
/// ```
pub fn combat_outcome2(input: &str) -> usize {
    let mut last_fail = 4;
    let mut max = 100;
    let mut current = last_fail;
    let mut count = 0;
    loop {
        count += 1;
        if count > 100 {
            return 0;
        }
        let mut game = Game::new(input, current);
        while game.next_round() {}
        if game
            .units
            .iter()
            .filter(|u| u.borrow().breed == "Elf" && u.borrow().hit_point() == 0)
            .count()
            == 0
        {
            if current <= (last_fail + 1) {
                return game.outcome();
            } else {
                max = current;
            }
            let revised = last_fail + (max - last_fail) / 2;
            if current == revised {
                current = revised - 1;
            } else {
                current = revised;
            }
        } else {
            last_fail = current;
            let revised = last_fail + (max - last_fail) / 2;
            if current == revised {
                current = revised + 1;
            } else {
                current = revised;
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(usize, usize);

impl Point {
    fn dist(&self, other: &Self) -> i64 {
        (self.0 as i64 - other.0 as i64).abs() + (self.1 as i64 - other.1 as i64).abs()
    }
}

struct Unit {
    hit_point: usize,
    breed: String,
    point: Point,
    power: usize,
}

impl Unit {
    fn init(ch: char, x: usize, y: usize, epower: usize) -> Option<Rc<RefCell<Unit>>> {
        let point = Point(x, y);
        match ch {
            'E' => Some(Rc::new(RefCell::new(Unit {
                hit_point: 200,
                breed: String::from("Elf"),
                point,
                power: epower,
            }))),
            'G' => Some(Rc::new(RefCell::new(Unit {
                hit_point: 200,
                breed: String::from("Goblin"),
                point,
                power: 3,
            }))),
            _ => None,
        }
    }

    fn hit_point(&self) -> usize {
        self.hit_point
    }

    fn take_hit(&mut self, power: usize) {
        if self.hit_point > power {
            self.hit_point -= power;
        } else {
            self.hit_point = 0;
        }
    }
}

fn cmp_point(a: &Rc<RefCell<Unit>>, b: &Rc<RefCell<Unit>>) -> std::cmp::Ordering {
    let x = a.borrow().point.1 * 10_000 + a.borrow().point.0;
    let y = b.borrow().point.1 * 10_000 + b.borrow().point.0;
    x.cmp(&y)
}

#[derive(Clone)]
enum Cell {
    Wall,
    OpenCavern,
    Occupied,
}

impl Cell {
    fn new(ch: char) -> Self {
        match ch {
            '#' => Cell::Wall,
            '.' => Cell::OpenCavern,
            'E' | 'G' => Cell::Occupied,
            _ => panic!("unknown cell type: {}", ch),
        }
    }
}

struct Game {
    cells: Vec<Vec<Cell>>,
    units: Vec<Rc<RefCell<Unit>>>,
    rounds: usize,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self
            .cells
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| match c {
                        Cell::Wall => '#',
                        Cell::OpenCavern => '.',
                        Cell::Occupied => 'O',
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect();
        let s2: String = self
            .units
            .iter()
            .map(|u| u.borrow().breed.clone() + "(" + &u.borrow().hit_point().to_string() + ") ")
            .collect();
        write!(f, "round: {}\n{}\n{}", self.rounds, s, s2)
    }
}

impl Game {
    fn new(s: &str, epower: usize) -> Self {
        Game {
            cells: s
                .trim()
                .split('\n')
                .map(|l| l.chars().map(Cell::new).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            units: s
                .trim()
                .split('\n')
                .enumerate()
                .map(|(i, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(j, c)| Unit::init(c, j, i, epower))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .filter_map(|u| u)
                .collect::<Vec<_>>(),
            rounds: 0,
        }
    }

    fn open_adjacencies(&self, point: &Point) -> Vec<Point> {
        vec![
            Point(point.0 - 1, point.1),
            Point(point.0 + 1, point.1),
            Point(point.0, point.1 - 1),
            Point(point.0, point.1 + 1),
        ]
        .into_iter()
        .filter(|p| {
            if let Cell::OpenCavern = self.cells[p.1][p.0] {
                true
            } else {
                false
            }
        })
        .collect::<Vec<_>>()
    }

    fn adjacencies(&self, point: &Point) -> Vec<Point> {
        vec![
            Point(point.0 - 1, point.1),
            Point(point.0 + 1, point.1),
            Point(point.0, point.1 - 1),
            Point(point.0, point.1 + 1),
        ]
    }

    fn successors(&self, point: &Point) -> Vec<(Point, u32)> {
        vec![
            Point(point.0 - 1, point.1),
            Point(point.0 + 1, point.1),
            Point(point.0, point.1 - 1),
            Point(point.0, point.1 + 1),
        ]
        .into_iter()
        .filter(|p| {
            if let Cell::OpenCavern = self.cells[p.1][p.0] {
                true
            } else {
                false
            }
        })
        .map(|p| (p, 1))
        .collect::<Vec<_>>()
    }

    fn next_round(&mut self) -> bool {
        self.units.sort_by(cmp_point);
        'outer: for u in self.units.iter().filter(|&x| x.borrow().hit_point() > 0) {
            let mut has_target = false;
            'target: for v in self
                .units
                .iter()
                .filter(|&x| x.borrow().breed != u.borrow().breed && x.borrow().hit_point() > 0)
            {
                for adj in self.adjacencies(&u.borrow().point) {
                    if adj == v.borrow().point {
                        has_target = true;
                        break 'target;
                    }
                }
            }

            if has_target {
                let (attacked, dead_point) = self.attack_target(u);
                if attacked {
                    if let Some(p) = dead_point {
                        self.cells[p.1][p.0] = Cell::OpenCavern;
                    }
                    continue 'outer;
                }
            }

            let mut ideal_path = vec![];
            let mut min_length = std::usize::MAX;
            let mut min_weight = std::usize::MAX;
            let enemies = self
                .units
                .iter()
                .filter(|&x| x.borrow().breed != u.borrow().breed && x.borrow().hit_point() > 0);
            if enemies.count() == 0 {
                return false;
            }
            let enemies = self
                .units
                .iter()
                .filter(|&x| x.borrow().breed != u.borrow().breed && x.borrow().hit_point() > 0);
            for v in enemies {
                for adj in self.open_adjacencies(&v.borrow().point) {
                    if let Some(path) = bfs(
                        &u.borrow().point,
                        |p| self.open_adjacencies(p),
                        |p| *p == adj,
                    ) {
                        let weight = adj.1 * 10_000 + adj.0;
                        if min_length > path.len() || (min_length == path.len() && min_weight > weight) {
                            min_length = path.len();
                            min_weight = weight;
                            ideal_path = path;
                        }
                    }
                }
            }

            if !ideal_path.is_empty() {
                let mut min_weight = std::usize::MAX;
                let mut new_point = Point(0, 0);

                let tcell = ideal_path.last().unwrap();
                if let Some((sol, _)) = astar_bag(
                    &u.borrow().point,
                    |p| self.successors(p),
                    |p| p.dist(tcell) as u32,
                    |p| *p == *tcell,
                ) {
                    for s in sol {
                        let p = s[1].clone();
                        let weight = p.1 * 10_000 + p.0;
                        if min_weight > weight {
                            min_weight = weight;
                            new_point = p;
                        }
                    }
                } else {
                    panic!("boom")
                }
                {
                    let old_point = &u.borrow().point;
                    self.cells[old_point.1][old_point.0] = Cell::OpenCavern;
                    self.cells[new_point.1][new_point.0] = Cell::Occupied;
                }
                u.borrow_mut().point = new_point.clone();
            }

            let (attacked, dead_point) = self.attack_target(u);
            if attacked {
                if let Some(p) = dead_point {
                    self.cells[p.1][p.0] = Cell::OpenCavern;
                }
            }
        }
        self.rounds += 1;
        true
    }

    fn attack_target(&self, u: &Rc<RefCell<Unit>>) -> (bool, Option<Point>) {
        let x = u.borrow().point.0;
        let y = u.borrow().point.1;
        let adjacent = vec![
            Point(x, y - 1),
            Point(x - 1, y),
            Point(x + 1, y),
            Point(x, y + 1),
        ];
        let mut target = std::usize::MAX;
        let mut min_hp = std::usize::MAX;
        for adj in adjacent {
            for (i, v) in self.units.iter().enumerate() {
                if v.borrow().point == adj
                    && v.borrow().breed != u.borrow().breed
                    && v.borrow().hit_point() > 0
                    && min_hp > v.borrow().hit_point() {
                        min_hp = v.borrow().hit_point();
                        target = i;
                }
            }
        }

        if target != std::usize::MAX {
            let t = &self.units[target];
            t.borrow_mut().take_hit(u.borrow().power);
            if t.borrow().hit_point() == 0 {
                let point = t.borrow().point.clone();
                return (true, Some(point));
            }
            (true, None)
        } else {
            (false, None)
        }
    }

    fn outcome(&self) -> usize {
        let mut hp = 0;
        for u in self.units.iter() {
            hp += u.borrow().hit_point();
        }
        self.rounds * hp
    }
}

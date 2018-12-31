/// Determine the resource value of the lumber collection area.
///
/// ```
/// use aoc18::day18::resource_value;
///
/// assert_eq!(1147, resource_value(".#.#...|#.
/// .....#|##|
/// .|..|...#.
/// ..|#.....#
/// #.#|||#|#|
/// ...#.||...
/// .|....|...
/// ||...#|.#|
/// |.||||..|.
/// ...#.|..|.
/// "));
/// ```
pub fn resource_value(input: &str) -> usize {
    let mut area: Vec<Vec<Acre>> = input
        .trim()
        .split('\n')
        .map(|l| l.chars().map(Acre::new).collect())
        .collect();
    let columns = area[0].len();
    for r in &mut area {
        r.insert(0, Acre::OpenGround);
        r.push(Acre::OpenGround);
    }
    area.insert(0, vec![Acre::OpenGround; columns + 2]);
    area.push(vec![Acre::OpenGround; columns + 2]);

    // let adjacencies = |r: usize, c: usize| -> Vec<&Acre> {
    //     vec![
    //         &area[r-1][c-1],
    //         &area[r-1][c],
    //         &area[r-1][c+1],
    //         &area[r][c-1],
    //         &area[r][c+1],
    //         &area[r+1][c-1],
    //         &area[r+1][c],
    //         &area[r+1][c+1],
    //     ]
    // };

    fn should_fill(adj: &[&Acre]) -> bool {
        adj.iter()
            .filter(|a| if let Acre::Trees = a { true } else { false })
            .count()
            >= 3
    }

    fn should_become_yard(adj: &[&Acre]) -> bool {
        adj.iter()
            .filter(|a| {
                if let Acre::Lumberyard = a {
                    true
                } else {
                    false
                }
            })
            .count()
            >= 3
    }

    fn should_remain_yard(adj: &[&Acre]) -> bool {
        adj.iter()
            .filter(|a| {
                if let Acre::Lumberyard = a {
                    true
                } else {
                    false
                }
            })
            .count()
            >= 1
            && adj
                .iter()
                .filter(|a| if let Acre::Trees = a { true } else { false })
                .count()
                >= 1
    }

    const COUNT: usize = 10;
    for _ in 0..COUNT {
        let new_area: Vec<Vec<Acre>> = area
            .iter()
            .enumerate()
            .map(|(i, r)| {
                r.iter()
                    .enumerate()
                    .map(|(j, a)| {
                        if i == 0 || j == 0 || i == columns + 1 || j == columns + 1 {
                            return a.clone();
                        }

                        let adj = vec![
                            &area[i - 1][j - 1],
                            &area[i - 1][j],
                            &area[i - 1][j + 1],
                            &area[i][j - 1],
                            &area[i][j + 1],
                            &area[i + 1][j - 1],
                            &area[i + 1][j],
                            &area[i + 1][j + 1],
                        ];
                        match a {
                            Acre::OpenGround => {
                                if should_fill(&adj) {
                                    Acre::Trees
                                } else {
                                    a.clone()
                                }
                            }
                            Acre::Trees => {
                                if should_become_yard(&adj) {
                                    Acre::Lumberyard
                                } else {
                                    a.clone()
                                }
                            }
                            Acre::Lumberyard => {
                                if should_remain_yard(&adj) {
                                    a.clone()
                                } else {
                                    Acre::OpenGround
                                }
                            }
                        }
                    })
                    .collect()
            })
            .collect();
        area = new_area;
    }

    let wooded = area
        .iter()
        .flatten()
        .filter(|a| if let Acre::Trees = a { true } else { false })
        .count();
    let yards = area
        .iter()
        .flatten()
        .filter(|a| {
            if let Acre::Lumberyard = a {
                true
            } else {
                false
            }
        })
        .count();
    wooded * yards
}

#[derive(Clone)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}

impl Acre {
    fn new(ch: char) -> Acre {
        match ch {
            '.' => Acre::OpenGround,
            '|' => Acre::Trees,
            '#' => Acre::Lumberyard,
            _ => panic!("boom"),
        }
    }
}

#[allow(dead_code)]
fn print_area(area: &Vec<Vec<Acre>>) {
    let repr: String = area
        .iter()
        .map(|r| {
            r.iter()
                .map(|a| match a {
                    Acre::OpenGround => '.',
                    Acre::Lumberyard => '#',
                    Acre::Trees => '|',
                })
                .collect::<String>()
                + "\n"
        })
        .collect();
    println!("{}", repr);
}

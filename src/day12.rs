use std::fmt;
use std::str::FromStr;

/// Find the sum of the numbers of all pots which contain a plant after given
/// number of generations.
pub fn sum_pots_after(input: &str, gen: usize) -> i64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut init_state_str = parts[0];
    init_state_str = init_state_str.trim_start_matches("initial state: ");
    let mut state = Row::from_str(init_state_str.trim()).unwrap();
    let mutations_str = parts[1];
    let mutations: Vec<Mutation> = mutations_str
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| Mutation::from_str(s).unwrap())
        .collect();
    const UPTO: usize = 1000;
    let offset = if gen > UPTO { gen - UPTO } else { 0 };
    for _ in 0..std::cmp::min(gen, UPTO) {
        state.mutate(&mutations);
    }
    state.start += offset as i64;
    state.state.iter().enumerate().fold(0, |acc, (i, x)| {
        if *x {
            acc + i as i64 + state.start
        } else {
            acc
        }
    })
}

#[test]
fn test_sum_pots_after() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
    assert_eq!(325, sum_pots_after(input, 20));
}

#[derive(Clone)]
struct Row {
    start: i64,
    state: Vec<bool>,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row {
            start: 0,
            state: s.chars().map(|c| c == '#').collect(),
        })
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self
            .state
            .iter()
            .map(|b| if *b { '#' } else { '.' })
            .collect();
        write!(f, "{} {}", self.start, s)
    }
}

impl Row {
    fn mutate(&mut self, mutations: &[Mutation]) {
        // Mark the positions of the first and last `true` value.
        let mut fs = 0;
        let mut ls = self.state.len() - 1;
        while !self.state[fs] {
            fs += 1;
        }
        while !self.state[ls] {
            ls -= 1;
        }

        // Trim excessive `false` values in the beginning and end, and keep only
        // 4 `false` values.
        let mut drained: Vec<_> = self.state.drain(fs..=ls).collect();
        self.state = vec![false; 4];
        self.state.append(&mut drained);
        self.state.append(&mut vec![false; 4]);
        self.start += fs as i64 - 4;

        let mut muts = Vec::new();
        for i in 2..self.state.len() - 2 {
            let t = vec![
                self.state[i - 2],
                self.state[i - 1],
                self.state[i],
                self.state[i + 1],
                self.state[i + 2],
            ];
            let mut found = false;
            for m in mutations {
                if m.state == t {
                    found = true;
                    muts.push((i, m.result));
                    break;
                }
            }
            if !found {
                muts.push((i, false));
            }
        }
        for (i, r) in muts {
            self.state[i] = r;
        }
    }
}

#[derive(Debug)]
struct Mutation {
    state: Vec<bool>,
    result: bool,
}

impl FromStr for Mutation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("=>").collect();
        let state_str = parts[0].trim();
        let result_str = parts[1].trim();
        let state: Vec<bool> = state_str.chars().map(|c| c == '#').collect();
        assert_eq!(state.len(), 5);
        Ok(Mutation {
            state,
            result: result_str == "#",
        })
    }
}

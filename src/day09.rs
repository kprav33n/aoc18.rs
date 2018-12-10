extern crate regex;

use self::regex::{Error, Regex};
use std::str::FromStr;
use std::collections::VecDeque;

/// Find the winning Elf's score.
///
/// # Examples
///
/// ```
/// use aoc18::day09::winning_score;
///
/// assert_eq!(32, winning_score("9 players; last marble is worth 25 points"));
/// assert_eq!(8317, winning_score("10 players; last marble is worth 1618 points"));
/// assert_eq!(146373, winning_score("13 players; last marble is worth 7999 points"));
/// assert_eq!(2764, winning_score("17 players; last marble is worth 1104 points"));
/// assert_eq!(54718, winning_score("21 players; last marble is worth 6111 points"));
/// assert_eq!(37305, winning_score("30 players; last marble is worth 5807 points"));
/// ```
pub fn winning_score(input: &str) -> usize {
    let mut g = Game::from_str(input.trim()).unwrap();
    g.play();
    g.scores.iter().fold(0, |max, x| if max < *x {*x} else {max})
}

/// Find the winning Elf's score.
///
/// # Examples
///
/// ```
/// use aoc18::day09::winning_score2;
///
/// assert_eq!(22563, winning_score2("9 players; last marble is worth 25 points"));
/// ```
pub fn winning_score2(input: &str) -> usize {
    let mut g = Game::from_str(input.trim()).unwrap();
    g.last_marble *= 100;
    g.play();
    g.scores.iter().fold(0, |max, x| if max < *x {*x} else {max})
}

#[derive(Debug)]
struct Game {
    num_players: usize,
    last_marble: usize,
    ring: Ring,
    scores: Vec<usize>,
    current_count: usize,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<num_players>[[:digit:]]+) players; last marble is worth (?P<last_marble>[[:digit:]]+) points").unwrap();
        let caps = re.captures(s).unwrap();
        let num_players = caps["num_players"].parse().unwrap();
        Ok(Game {
            num_players,
            last_marble: caps["last_marble"].parse().unwrap(),
            ring: Ring::new(),
            scores: vec![0; num_players],
            current_count: 1,
        })
    }
}

impl Game {
    fn next_step(&mut self) {
        // NOTE: This solution is not going to scale as it uses vector instead
        // of linked list.
        if self.current_count % 23 == 0 {
            for _ in 0..7 {
                self.ring.rotate_right();
            }
            let v = self.ring.buffer.pop_front().unwrap();
            let current_player = self.current_count % self.scores.len();
            self.scores[current_player] += self.current_count;
            self.scores[current_player] += v;
        } else {
            self.ring.rotate_left();
            self.ring.rotate_left();
            self.ring.buffer.push_front(self.current_count);
        }
        self.current_count += 1;
    }

    fn play(&mut self) {
        for _ in 0..self.last_marble {
            self.next_step();
        }
    }
}

#[derive(Debug)]
struct Ring {
    buffer: VecDeque<usize>
}

impl Ring {
    fn new() -> Self {
        let mut r = Ring{buffer: VecDeque::new()};
        r.buffer.push_back(0);
        r
    }

    fn rotate_right(&mut self) {
        let v = self.buffer.pop_back().unwrap();
        self.buffer.push_front(v);
    }

    fn rotate_left(&mut self) {
        let v = self.buffer.pop_front().unwrap();
        self.buffer.push_back(v);
    }
}

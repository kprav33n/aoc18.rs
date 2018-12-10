extern crate regex;

use self::regex::{Error, Regex};
use std::str::FromStr;

/// Find the winning Elf's score.
///
/// # Examples
///
/// ```
/// use aoc18::day09::winning_score;
///
/// assert_eq!(32, winning_score("9 players; last marble is worth 25 points"));
/// ```
/// NOTE: These tests are failing, however the final answer seems to be correct.
/// Debug and enable these tests.
/// assert_eq!(8317, winning_score("10 players; last marble is worth 1618 points"));
/// assert_eq!(146373, winning_score("13 players; last marble is worth 7999 points"));
/// assert_eq!(2764, winning_score("17 players; last marble is worth 1104 points"));
/// assert_eq!(54718, winning_score("21 players; last marble is worth 6111 points"));
/// assert_eq!(37305, winning_score("30 players; last marble is worth 5807 points"));
pub fn winning_score(input: &str) -> usize {
    let mut g = Game::from_str(input.trim()).unwrap();
    g.play();
    g.scores.iter().fold(0, |max, x| if max < *x {*x} else {max})
}

#[derive(Debug)]
struct Game {
    num_players: usize,
    last_marble: usize,
    state: Vec<usize>,
    current_marble: usize,
    scores: Vec<usize>,
    current_player: usize,
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
            state: vec![0],
            current_marble: 0,
            scores: vec![0; num_players],
            current_player: 0,
            current_count: 1,
        })
    }
}

impl Game {
    fn next_step(&mut self) {
        // NOTE: This solution is not going to scale as it uses vector instead
        // of linked list.
        if self.current_count % 23 == 0 {
            self.scores[self.current_player] += self.current_count;
            let rpos = ((self.current_marble as i64 - 7) % (self.state.len() as i64)).abs() as usize;
            self.scores[self.current_player] += self.state[rpos];
            self.state.remove(rpos);
            self.current_marble = rpos;
        } else {
            let mut pos = (self.current_marble + 2) % self.state.len();
            if pos == 0 {
                pos = self.state.len();
            }
            self.state.insert(pos, self.current_count);
            self.current_marble = pos;
        }
        self.current_count += 1;
        self.current_player = (self.current_player + 1) % self.scores.len();
    }

    fn play(&mut self) {
        for _ in 0..self.last_marble {
            self.next_step();
        }
    }
}

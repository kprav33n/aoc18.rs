extern crate petgraph;
extern crate regex;

use self::petgraph::graphmap::DiGraphMap;
use self::regex::{Error, Regex};
use std::str::FromStr;
use std::collections::BTreeSet;

/// Find the size of the largest finite area based on given list of coordinates.
///
/// # Examples
///
/// ```
/// use aoc18::day07::steps;
///
/// assert_eq!("CABDFE", steps("
/// Step C must be finished before step A can begin.
/// Step C must be finished before step F can begin.
/// Step A must be finished before step B can begin.
/// Step A must be finished before step D can begin.
/// Step B must be finished before step E can begin.
/// Step D must be finished before step E can begin.
/// Step F must be finished before step E can begin.
/// "));
/// ```
pub fn steps(input: &str) -> String {
    let mut deps = DiGraphMap::new();
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| OrderingEntry::from_str(s).unwrap())
        .for_each(|e| {
            deps.add_edge(e.first, e.second, 1);
        });

    // NOTE: Topological sort, Kahn's algorithm.
    // https://en.wikipedia.org/wiki/Topological_sorting
    let mut s = BTreeSet::new();
    'outer: for n in deps.nodes() {
        for _ in deps.neighbors_directed(n, petgraph::Incoming) {
            continue 'outer;
        }
        s.insert(n);
    }

    let mut l = Vec::new();
    while !s.is_empty() {
        let n = *s.iter().next().unwrap();
        s.remove(&n);
        l.push(n);

        let mut removals = Vec::new();
        for m in deps.neighbors_directed(n, petgraph::Outgoing) {
            removals.push((n, m));
        }

        'removal: for r in removals {
            deps.remove_edge(r.0, r.1);
            for _ in deps.neighbors_directed(r.1, petgraph::Incoming) {
                continue 'removal;
            }
            s.insert(r.1);
        }
    }

    l.into_iter().collect()
}

#[derive(Debug, PartialEq)]
struct OrderingEntry {
    first: char,
    second: char,
}

impl FromStr for OrderingEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Step (?P<first>[[:alpha:]]) must be finished before step (?P<second>[[:alpha:]]) can begin.").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(OrderingEntry {
            first: caps["first"].chars().next().unwrap(),
            second: caps["second"].chars().next().unwrap(),
        })
    }
}

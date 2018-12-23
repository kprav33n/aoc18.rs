use petgraph::graphmap::DiGraphMap;
use regex::{Error, Regex};
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::mpsc;
use std::{thread, time};

/// Find the order of steps that will be executed from the given instructions.
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
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| OrderingEntry::from_str(s).unwrap())
        .for_each(|e| {
            deps.add_edge(e.first, e.second, 1);
        });

    // NOTE: Topological sort, Kahn's algorithm.
    // https://en.wikipedia.org/wiki/Topological_sorting
    let mut s = BTreeSet::new();
    for n in deps.nodes() {
        if deps
            .neighbors_directed(n, petgraph::Incoming)
            .next()
            .is_none()
        {
            s.insert(n);
        }
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

        for r in removals {
            deps.remove_edge(r.0, r.1);
            if deps
                .neighbors_directed(r.1, petgraph::Incoming)
                .next()
                .is_none()
            {
                s.insert(r.1);
            }
        }
    }

    l.into_iter().collect()
}

/// Find the time it will take to complete the order of steps that will be
/// executed from the given instructions, using a given number of workers.
///
/// # Examples
///
/// ```
/// use aoc18::day07::completion_time;
///
/// assert_eq!(15, completion_time("
/// Step C must be finished before step A can begin.
/// Step C must be finished before step F can begin.
/// Step A must be finished before step B can begin.
/// Step A must be finished before step D can begin.
/// Step B must be finished before step E can begin.
/// Step D must be finished before step E can begin.
/// Step F must be finished before step E can begin.
/// ", 2, 0));
/// ```

pub fn completion_time(input: &str, workers: usize, factor: usize) -> usize {
    let mut deps = DiGraphMap::new();
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| OrderingEntry::from_str(s).unwrap())
        .for_each(|e| {
            deps.add_edge(e.first, e.second, 1);
        });
    let count = deps.nodes().fold(0, |acc, _| acc + 1);

    let (tx, rx) = Crew::init(workers, factor);

    // NOTE: Topological sort, Kahn's algorithm.
    // https://en.wikipedia.org/wiki/Topological_sorting
    let mut s = BTreeSet::new();
    for n in deps.nodes() {
        if deps
            .neighbors_directed(n, petgraph::Incoming)
            .next()
            .is_none()
        {
            s.insert(n);
        }
    }

    let mut l = Vec::new();
    let mut ticks = 0;
    // loop here, if s in non_empty, push to the crew, else wait for the crew.
    while l.len() < count {
        match s.iter().next() {
            Some(&n) => {
                s.remove(&n);
                let _ = tx.send(n);
            }

            _ => {
                if let Ok((n, t)) = rx.try_recv() {
                    ticks = t;
                    l.push(n);
                    let mut removals = Vec::new();
                    for m in deps.neighbors_directed(n, petgraph::Outgoing) {
                        removals.push((n, m));
                    }

                    for r in removals {
                        deps.remove_edge(r.0, r.1);
                        if deps
                            .neighbors_directed(r.1, petgraph::Incoming)
                            .next()
                            .is_none()
                        {
                            s.insert(r.1);
                        }
                    }
                }
            }
        }
    }

    ticks + 1
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

#[derive(Debug)]
struct Crew {
    factor: usize,
    workers: Vec<(char, usize)>,
    idle: Vec<usize>,
    backlog: Vec<char>,
    ticks: usize,
    output: mpsc::Sender<(char, usize)>,
    input: mpsc::Receiver<char>,
}

impl Crew {
    fn init(size: usize, factor: usize) -> (mpsc::Sender<char>, mpsc::Receiver<(char, usize)>) {
        let (tx_in, rx_in) = mpsc::channel();
        let (tx_out, rx_out) = mpsc::channel();
        let mut crew = Crew {
            factor,
            workers: vec![('U', 0); size],
            idle: (0..size).rev().collect(),
            backlog: vec![],
            ticks: 0,
            output: tx_out,
            input: rx_in,
        };
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(1));
            match crew.input.try_recv() {
                Ok(item) => crew.new_item(item),
                _ => {
                    crew.new_tick();
                }
            }
        });
        (tx_in, rx_out)
    }

    fn new_item(&mut self, item: char) {
        if !self.idle.is_empty() {
            let i = self.idle.pop().unwrap();
            self.assign_item(i, item);
        } else {
            self.backlog.push(item);
        }
    }

    fn new_tick(&mut self) {
        for i in 0..self.workers.len() {
            match self.workers[i] {
                (_, 0) => {
                    // Waiting idle.
                    if let Some(c) = self.backlog.pop() {
                        self.assign_item(i, c);
                    }
                }

                (c, 1) => {
                    // Completed.
                    let _ = self.output.send((c, self.ticks));
                    match self.backlog.pop() {
                        Some(item) => {
                            self.assign_item(i, item);
                        }
                        _ => {
                            self.workers[i] = (c, 0);
                            self.idle.push(i);
                        }
                    }
                }

                (c, n) => {
                    // Busy.
                    self.workers[i] = (c, n - 1);
                }
            }
        }
        self.ticks += 1;
    }

    fn assign_item(&mut self, worker: usize, item: char) {
        self.workers[worker] = (item, item as usize - 'A' as usize + 1 + self.factor)
    }
}

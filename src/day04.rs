extern crate chrono;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use self::chrono::Timelike;

/// Compute the product of id of guard who sleeps the most and his most slept
/// minute.
///
/// # Examples
///
/// ```
/// use aoc18::day04::slacker_id_min;
///
/// assert_eq!(240, slacker_id_min("[1518-11-01 00:00] Guard #10 begins shift
/// [1518-11-01 00:05] falls asleep
/// [1518-11-01 00:25] wakes up
/// [1518-11-01 00:30] falls asleep
/// [1518-11-01 00:55] wakes up
/// [1518-11-01 23:58] Guard #99 begins shift
/// [1518-11-02 00:40] falls asleep
/// [1518-11-02 00:50] wakes up
/// [1518-11-03 00:05] Guard #10 begins shift
/// [1518-11-03 00:24] falls asleep
/// [1518-11-03 00:29] wakes up
/// [1518-11-04 00:02] Guard #99 begins shift
/// [1518-11-04 00:36] falls asleep
/// [1518-11-04 00:46] wakes up
/// [1518-11-05 00:03] Guard #99 begins shift
/// [1518-11-05 00:45] falls asleep
/// [1518-11-05 00:55] wakes up"));
/// ```
pub fn slacker_id_min(input: &str) -> u64 {
    let mut entries = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| entry(s.trim()).unwrap())
        .map(|(_, e)| e)
        .collect::<Vec<Event>>();
    entries.sort();
    let mut current_id = 0;
    // NOTE: RefCell<T> and the Interior Mutability Pattern
    // https://doc.rust-lang.org/book/second-edition/ch15-05-interior-mutability.html
    let mut guards: HashMap<u64, Rc<RefCell<Vec<u64>>>> = HashMap::new();
    for entry in entries {
        match entry {
            Event::Begin(id, _) => {
                current_id = id;
                guards.entry(id).or_insert(Rc::new(RefCell::new(vec![0; 60])));
            }
            Event::Sleep(t) => {
                let mut guard = guards.get_mut(&current_id).unwrap().borrow_mut();
                for i in t.time().minute()..60 {
                    guard[i as usize] += 1;
                }
            }
            Event::Wake(t) => {
                let mut guard = guards.get_mut(&current_id).unwrap().borrow_mut();
                for i in t.time().minute()..60 {
                    guard[i as usize] -= 1;
                }
            }
        }
    }
    let mut slacker = 0;
    let mut max_sleep = 0;
    let mut minute: u64 = 0;
    for (key, value) in guards {
        let sum: u64 = value.borrow().iter().sum();
        if max_sleep < sum {
            max_sleep = sum;
            slacker = key;
            minute = (value.borrow().iter().enumerate().max_by_key(|&(_, item)| item).unwrap().0) as u64
        }

    }
    minute * slacker
}

type DateTime = chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq)]
enum Event {
    Begin(u64, DateTime),
    Sleep(DateTime),
    Wake(DateTime),
}

fn event_time(e: &Event) -> &DateTime {
    match e {
        Event::Begin(_, t) => t,
        Event::Sleep(t) => t,
        Event::Wake(t) => t
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> std::cmp::Ordering {
        let t1 = event_time(self);
        let t2 = event_time(other);
        t1.cmp(&t2)
    }
}

named!(
    number<&str, u64>,
    map_res!(
        recognize!(nom::digit),
        |s: &str| s.parse()
    )
);

named!(
    time<&str, DateTime>,
    map_res!(
        take_while_m_n!(16, 16, |_| true),
        |s: &str| DateTime::parse_from_str(s, "%Y-%m-%d %H:%M")
    )
);

named!(
    begin<&str, Event>,
    do_parse!(
        tag!("[") >>
        t: time >>
        tag!("] Guard #") >>
        id: number >>
        tag!(" begins shift") >>
        (Event::Begin(id, t))
    )
);

named!(
    sleep<&str, Event>,
    do_parse!(
        tag!("[") >>
        t: time >>
        tag!("] falls asleep") >>
        (Event::Sleep(t))
    )
);

named!(
    wake<&str, Event>,
    do_parse!(
        tag!("[") >>
        t: time >>
        tag!("] wakes up") >>
        (Event::Wake(t))
    )
);

named!(
    entry<&str, Event>,
    alt!(begin | sleep | wake)
);

#[test]
fn parse_time() {
    let r = time("1518-10-31 00:58");
    match r {
        Ok((rest, t)) => {
            assert_eq!(rest, "");
            assert_eq!(
                t,
                chrono::NaiveDate::from_ymd(1518, 10, 31).and_hms(0, 58, 0)
            );
        }
        Err(e) => {
            println!("error while parsing : {}", e);
            assert!(false);
        }
    }
}

#[test]
fn parse_begin() {
    let b = begin("[1518-11-01 23:58] Guard #99 begins shift");
    match b {
        Ok((rest, e)) => match e {
            Event::Begin(id, t) => {
                assert_eq!(rest, "");
                assert_eq!(id, 99);
                assert_eq!(
                    t,
                    chrono::NaiveDate::from_ymd(1518, 11, 1).and_hms(23, 58, 0)
                );
            }
            _ => {
                assert!(false);
            }
        },
        Err(e) => {
            println!("error while parsing : {}", e);
            assert!(false);
        }
    }
}

#[test]
fn parse_sleep() {
    let b = sleep("[1518-10-17 00:51] falls asleep");
    match b {
        Ok((rest, e)) => match e {
            Event::Sleep(t) => {
                assert_eq!(rest, "");
                assert_eq!(
                    t,
                    chrono::NaiveDate::from_ymd(1518, 10, 17).and_hms(00, 51, 0)
                );
            }
            _ => {
                assert!(false);
            }
        },
        Err(e) => {
            println!("error while parsing : {}", e);
            assert!(false);
        }
    }
}

#[test]
fn parse_wake() {
    let b = wake("[1518-11-18 00:41] wakes up");
    match b {
        Ok((rest, e)) => match e {
            Event::Wake(t) => {
                assert_eq!(rest, "");
                assert_eq!(
                    t,
                    chrono::NaiveDate::from_ymd(1518, 11, 18).and_hms(00, 41, 0)
                );
            }
            _ => {
                assert!(false);
            }
        },
        Err(e) => {
            println!("error while parsing : {}", e);
            assert!(false);
        }
    }
}

#[test]
fn parse_entry() {
    let b = begin("[1518-11-01 23:58] Guard #99 begins shift");
    assert_eq!(
        b,
        Ok((
            "",
            Event::Begin(
                99,
                chrono::NaiveDate::from_ymd(1518, 11, 01).and_hms(23, 58, 0)
            )
        ))
    );
    let s = sleep("[1518-10-17 00:51] falls asleep");
    assert_eq!(
        s,
        Ok((
            "",
            Event::Sleep(chrono::NaiveDate::from_ymd(1518, 10, 17).and_hms(00, 51, 0))
        ))
    );
    let w = entry("[1518-11-18 00:41] wakes up");
    assert_eq!(
        w,
        Ok((
            "",
            Event::Wake(chrono::NaiveDate::from_ymd(1518, 11, 18).and_hms(00, 41, 0))
        ))
    );
}

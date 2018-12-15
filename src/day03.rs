use nom::{call, do_parse, error_position, map_res, named, recognize, tag};

/// Compute overlapping area for the given list of claims.
///
/// # Examples
///
/// ```
/// use aoc18::day03::overlapping_area;
///
/// assert_eq!(4, overlapping_area("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
/// ```
pub fn overlapping_area(input: &str) -> u64 {
    const BOUND: usize = 1000;
    let mut fabric = [[0u8; BOUND]; BOUND];
    for line in input.split('\n') {
        // NOTE: Parsing the exact input line has a gotcha while using nom 4.
        // Hence, append a space at the end of the line to make parser happy.
        // https://github.com/Geal/nom/issues/764
        let mut l = String::from(line.trim());
        if l.is_empty() {
            continue;
        }
        l.push(' ');
        match claim(l.as_str()) {
            Ok((_, c)) => {
                for i in c.left_offset..c.left_offset + c.width {
                    for f in fabric.iter_mut().skip(c.top_offset).take(c.height) {
                        f[i] += 1;
                    }
                }
            }
            Err(e) => println!("error while parsing {}: {}", l, e),
        }
    }
    let mut result = 0;
    for i in 0..BOUND {
        for f in fabric.iter().take(BOUND) {
            if f[i] > 1 {
                result += 1;
            }
        }
    }
    result
}

/// Compute overlapping area for the given list of claims.
///
/// # Examples
///
/// ```
/// use aoc18::day03::intact_claim;
///
/// assert_eq!(3, intact_claim("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
/// ```
pub fn intact_claim(input: &str) -> usize {
    const BOUND: usize = 1000;
    let mut fabric = [[0u8; BOUND]; BOUND];
    let mut claims = Vec::new();
    for line in input.split('\n') {
        // NOTE: Parsing the exact input line has a gotcha while using nom 4.
        // Hence, append a space at the end of the line to make parser happy.
        // https://github.com/Geal/nom/issues/764
        let mut l = String::from(line.trim());
        if l.is_empty() {
            continue;
        }
        l.push(' ');
        match claim(l.as_str()) {
            Ok((_, c)) => {
                for i in c.left_offset..c.left_offset + c.width {
                    for f in fabric.iter_mut().skip(c.top_offset).take(c.height) {
                        f[i] += 1;
                    }
                }
                claims.push(c);
            }
            Err(e) => println!("error while parsing {}: {}", l, e),
        }
    }

    'outer: for c in claims {
        for i in c.left_offset..c.left_offset + c.width {
            for f in fabric.iter().skip(c.top_offset).take(c.height) {
                if f[i] > 1 {
                    continue 'outer;
                }
            }
        }
        return c.id;
    }
    0
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left_offset: usize,
    top_offset: usize,
    width: usize,
    height: usize,
}

named!(
    number<&str, usize>,
    map_res!(
        recognize!(nom::digit),
        |s: &str| s.parse()
    )
);

named!(
    claim<&str, Claim>,
    do_parse!(
        tag!("#") >>
        id: number >>
        tag!(" @ ") >>
        left_offset: number >>
        tag!(",") >>
        top_offset: number >>
        tag!(": ") >>
        width: number >>
        tag!("x") >>
        height: number >>
        (Claim {id, left_offset, top_offset, width, height})
    )
);

#[test]
fn parse_claim() {
    assert_eq!(number("123 @ "), Ok((" @ ", 123)));
    assert_eq!(
        claim("#123 @ 3,2: 5x4 "),
        Ok((
            " ",
            Claim {
                id: 123,
                left_offset: 3,
                top_offset: 2,
                width: 5,
                height: 4
            }
        ))
    );
}

use nom::{call, char, do_parse, error_position, fold_many_m_n, map_res, named, recognize};
/// Find the sum of all metadata entries.
///
/// # Examples
///
/// ```
/// use aoc18::day08::meta_sum;
///
/// assert_eq!(138, meta_sum("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2 "));
/// ```
pub fn meta_sum(input: &str) -> usize {
    // NOTE: Passing the exact string results in parser error
    // `Err(Incomplete(Size(1)))`. So, pad-right the test input by a space, and
    // pad-right the puzzle input by a `\n`.
    match node(input) {
        Ok((_, n)) => n.meta_sum(),

        x => {
            println!("{:?}", x);
            0
        }
    }
}

/// Find the value of the root node.
///
/// # Examples
///
/// ```
/// use aoc18::day08::root_value;
///
/// assert_eq!(66, root_value("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2 "));
/// ```
pub fn root_value(input: &str) -> usize {
    // NOTE: Passing the exact string results in parser error
    // `Err(Incomplete(Size(1)))`. So, pad-right the test input by a space, and
    // pad-right the puzzle input by a `\n`.
    match node(input) {
        Ok((_, n)) => n.value(),

        x => {
            println!("{:?}", x);
            0
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn meta_sum(&self) -> usize {
        self.children
            .iter()
            .fold(self.metadata.iter().sum(), |acc, c| acc + c.meta_sum())
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter().fold(0, |acc, i| {
                if *i == 0 || *i > self.children.len() {
                    acc
                } else {
                    acc + self.children[i - 1].value()
                }
            })
        }
    }
}

named!(
    number<&str, usize>,
    map_res!(
        recognize!(nom::digit),
        |s: &str| s.parse()
    )
);

named!(
    node<&str, Node>,
    do_parse!(
        num_children: number >>
            char!(' ') >>
            num_metadata: number >>
            children: fold_many_m_n!(num_children, num_children, do_parse!(char!(' ') >> n: node >> (n)), Vec::new(), |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            }) >>
            metadata: fold_many_m_n!(num_metadata, num_metadata, do_parse!(char!(' ') >> n: number >> (n)), Vec::new(), |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            }) >>
            (Node{children, metadata})
    )
);

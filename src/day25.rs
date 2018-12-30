use petgraph::algo::connected_components;
use petgraph::graphmap::UnGraphMap;
use std::str::FromStr;

/// Find the number of constellations formed by the given points.
pub fn num_constellations(input: &str) -> usize {
    let points: Vec<Point4D> = input
        .trim()
        .split('\n')
        .map(|s| Point4D::from_str(s).unwrap())
        .collect();

    let mut graph = UnGraphMap::new();
    for i in 0..points.len() {
        graph.add_node(i);
    }
    for i in 0..points.len() - 1 {
        for j in (i + 1)..points.len() {
            if points[i].dist(&points[j]) <= 3 {
                graph.add_edge(i, j, 1);
            }
        }
    }
    connected_components(&graph)
}

struct Point4D {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl FromStr for Point4D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ps: Vec<i64> = s.trim().split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Point4D {
            a: ps[0],
            b: ps[1],
            c: ps[2],
            d: ps[3],
        })
    }
}

impl Point4D {
    fn dist(&self, other: &Self) -> i64 {
        (self.a - other.a).abs()
            + (self.b - other.b).abs()
            + (self.c - other.c).abs()
            + (self.d - other.d).abs()
    }
}

#[test]
fn test_num_constellations1() {
    assert_eq!(
        2,
        num_constellations(
            " 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0
"
        )
    );
}

#[test]
fn test_num_constellations2() {
    assert_eq!(
        4,
        num_constellations(
            "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
"
        )
    );
}

#[test]
fn test_num_constellations3() {
    assert_eq!(
        3,
        num_constellations(
            "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
"
        )
    );
}

#[test]
fn test_num_constellations4() {
    assert_eq!(
        8,
        num_constellations(
            "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
"
        )
    );
}

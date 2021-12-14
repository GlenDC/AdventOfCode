use std::collections::HashMap;

fn main() {
    println!("++++++++++++");
    println!("PART#1");
    println!("++++++++++++");

    part1(TEST_INPUT);

    println!("-----");

    part1(INPUT);

    println!("++++++++++++");
    println!("PART#2");
    println!("++++++++++++");

    part2(TEST_INPUT);

    println!("-----");

    part2(INPUT);
}

fn part1(input: &str) {
    // register all points & connections
    let mut cave_map = CaveMap::new();
    for line in input.split("\n") {
        let mut it = line.split("-");
        let a: Point = it.next().unwrap().into();
        let b: Point = it.next().unwrap().into();
        assert!(it.next().is_none());
        cave_map.register_conn(a, b);
    }
    // println!("{:?}", cave_map);

    let mut paths_wip = Vec::new();
    let mut paths_successful = Vec::new();

    // start from start & build up from there
    for conn in cave_map.connections_from(&Point::Start).unwrap() {
        let mut p = Path::new();
        assert!(p.extend(conn));
        paths_wip.push(p);
    }

    // fill all paths, until nothing is left as wip...
    loop {
        match paths_wip.pop() {
            None => break,
            Some(path) => match cave_map.connections_from(path.tail()) {
                None => {
                    // path has dead end, dropping it
                    // println!("path dropped due dead end, path: {:?}", path);
                    continue;
                }
                Some(conns) => {
                    for conn in conns {
                        let mut path_cloned = path.clone();
                        match conn {
                            Point::End => {
                                assert!(path_cloned.extend(conn));
                                // println!("successful path: {:?}", path_cloned);
                                paths_successful.push(path_cloned);
                                continue;
                            }
                            _ => {
                                if path_cloned.extend(conn) {
                                    paths_wip.push(path_cloned);
                                    continue;
                                }
                                // drop path, path cannot be extended due to double crossing of small cave
                                // println!("path dropped due to double crossing small cave ({:?}), path: {:?}", conn, path_cloned);
                            }
                        }
                    }
                }
            },
        }
    }

    println!("{}", paths_successful.len());
}

fn part2(input: &str) {
    // register all points & connections
    let mut cave_map = CaveMap::new();
    for line in input.split("\n") {
        let mut it = line.split("-");
        let a: Point = it.next().unwrap().into();
        let b: Point = it.next().unwrap().into();
        assert!(it.next().is_none());
        cave_map.register_conn(a, b);
    }
    // println!("{:?}", cave_map);

    let mut paths_wip = Vec::new();
    let mut paths_successful = Vec::new();

    // start from start & build up from there
    for conn in cave_map.connections_from(&Point::Start).unwrap() {
        let mut p = Path::new_part2();
        assert!(p.extend(conn));
        paths_wip.push(p);
    }

    // fill all paths, until nothing is left as wip...
    loop {
        match paths_wip.pop() {
            None => break,
            Some(path) => match cave_map.connections_from(path.tail()) {
                None => {
                    // path has dead end, dropping it
                    // println!("path dropped due dead end, path: {:?}", path);
                    continue;
                }
                Some(conns) => {
                    for conn in conns {
                        let mut path_cloned = path.clone();
                        match conn {
                            Point::End => {
                                assert!(path_cloned.extend(conn));
                                // println!("successful path: {:?}", path_cloned);
                                paths_successful.push(path_cloned);
                                continue;
                            }
                            _ => {
                                if path_cloned.extend(conn) {
                                    paths_wip.push(path_cloned);
                                    continue;
                                }
                                // drop path, path cannot be extended due to double crossing of small cave
                                // println!("path dropped due to double crossing small cave ({:?}), path: {:?}", conn, path_cloned);
                            }
                        }
                    }
                }
            },
        }
    }

    println!("{}", paths_successful.len());
}

// NOTE: this can be made more memory-efficient by storing the id of a Small-/Big- Cave
// as an u16, by converting s=[b0,b1] as (b0<<5 | b1). However, for this simple toy challenge it is not worth the effort.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Point {
    Start,
    End,
    SmallCave(String),
    BigCave(String),
}

impl From<&str> for Point {
    fn from(s: &str) -> Point {
        match s.trim() {
            "start" => Point::Start,
            "end" => Point::End,
            s => {
                if s.bytes().all(|b| matches!(b, b'a'..=b'z')) {
                    Point::SmallCave(String::from(s))
                } else {
                    Point::BigCave(String::from(s))
                }
            }
        }
    }
}

#[derive(Debug)]
struct CaveMap {
    connections: HashMap<Point, Vec<Point>>,
}

impl CaveMap {
    pub fn new() -> CaveMap {
        CaveMap {
            connections: HashMap::new(),
        }
    }

    pub fn register_conn(&mut self, a: Point, b: Point) {
        self._add_connection(a.clone(), b.clone());
        self._add_connection(b, a);
    }

    fn _add_connection(&mut self, a: Point, b: Point) {
        let v = self.connections.entry(a).or_insert(Vec::new());
        if v.iter().any(|p| *p == b) {
            return;
        }
        v.push(b);
    }

    pub fn connections_from(&self, p: &Point) -> Option<Vec<&Point>> {
        self.connections
            .get(p)
            .and_then(|v| Some(v.iter().collect()))
    }
}

// NOTE: this can be probably done without any cloning, by working with something like:
// ```
// struct PathNode<'a> {
//     point: Point,
//     parent: Option<&'a PathNode<'a>>,
// }
// ```
// To make that work without ever needing to clone however would probably want to use some kind of std::rc::Rc
// smart pointer to be able to work with references and at the same time still making sure the actual memory lives long
// enough without having to store it somewhere explicitly. Could be fun to do, but not my cup of tea for this exercise.
#[derive(Debug, Clone)]
struct Path {
    points: Vec<Point>,
    // used for part #2
    allow_small_cave_double_visit: bool,
}

impl Path {
    pub fn new() -> Path {
        Path {
            points: vec![Point::Start],
            allow_small_cave_double_visit: false,
        }
    }

    pub fn new_part2() -> Path {
        Path {
            points: vec![Point::Start],
            allow_small_cave_double_visit: true,
        }
    }

    // NOTE:
    // Can be made more efficient by splitting it up as
    // ```
    // fn extend_as(&mut self, p: &Point) -> Option<Path>
    // ```
    // and
    // ```
    // fn extend_into(self, p: &Point) -> Option<Path>
    // ```
    pub fn extend(&mut self, p: &Point) -> bool {
        match p {
            Point::BigCave(_) => (),
            Point::SmallCave(_) => {
                if self.points.iter().any(|c| c == p) {
                    if !self.allow_small_cave_double_visit {
                        return false;
                    }
                    self.allow_small_cave_double_visit = false;
                }
            }
            _ => {
                if self.points.iter().any(|c| c == p) {
                    return false;
                }
            }
        }
        self.points.push(p.clone());
        true
    }

    pub fn tail(&self) -> &Point {
        &self.points[self.points.len() - 1]
    }
}

const TEST_INPUT: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const INPUT: &'static str = "fw-ll
end-dy
tx-fw
tx-tr
dy-jb
ZD-dy
dy-BL
dy-tr
dy-KX
KX-start
KX-tx
fw-ZD
tr-end
fw-jb
fw-yi
ZD-nr
start-fw
tx-ll
ll-jb
yi-jb
yi-ll
yi-start
ZD-end
ZD-jb
tx-ZD";

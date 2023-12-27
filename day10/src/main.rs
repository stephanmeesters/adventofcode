use array2d::Array2D;
use std::fs;
use std::io::Write;
use std::ops;

use queues::*;

macro_rules! getchar {
    ($a:expr,$b:expr) => {{
        $a[$b.x].chars().nth($b.y).unwrap()
    }};
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl ops::Add<&Pos> for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Visited(usize),
    Unvisited,
}

fn main() {
    println!("result: {}", run_test("input.txt"));
}

fn run_test(fname: &str) -> usize {
    let contents = fs::read_to_string(fname).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut candidates: Queue<Pos> = queue![];

    let mut map = Array2D::filled_with(Node::Unvisited, lines.len(), lines[0].len());
    let start_index: Pos = get_start_index(&lines).unwrap();
    map[(start_index.x, start_index.y)] = Node::Visited(0);
    let _ = candidates.add(start_index);

    while candidates.size() > 0 {
        let candidate = candidates.remove().unwrap();
        let cur_str = getchar!(lines, candidate);
        if cur_str == '.' {
            continue;
        }
        println!("point {:?} {}", candidate, cur_str);

        for neigh in get_neighbors(&lines, &candidate) {
            let neigh_str = getchar!(lines, neigh);
            println!("{:?} {}", candidate, cur_str);
            if neigh_str == '.' {
                continue;
            }
            println!("{:?} {}", candidate, cur_str);
            match map[(candidate.x, candidate.y)] {
                Node::Visited(val) => match map[(neigh.x, neigh.y)] {
                    Node::Visited(nval) => {
                        if (val + 1) < nval && nval != 0 {
                            map[(neigh.x, neigh.y)] = Node::Visited(val + 1);
                            let _ = candidates.add(neigh);
                        }
                    }
                    Node::Unvisited => {
                        map[(neigh.x, neigh.y)] = Node::Visited(val + 1);
                        let _ = candidates.add(neigh);
                    }
                },
                _ => (),
            }
        }
    }

    let mut maxsize: usize = 0;

    // Open a file in write mode
    let mut file = fs::File::create("output.txt").unwrap();

    for row_iter in map.rows_iter() {
        for element in row_iter {
            match element {
                Node::Visited(val) => {
                    // Write to file instead of printing
                    if val > &maxsize {
                        maxsize = *val;
                    }
                    write!(file, "{:>5}", val);
                }
                _ => {
                    // Write a dot for other cases
                    write!(file, "{:>5}", ".");
                }
            }
        }
        // Write a newline character at the end of each row
        writeln!(file);
    }

    maxsize
}

fn get_start_index(lines: &Vec<&str>) -> Option<Pos> {
    for i in 0..lines.len() {
        let line = lines[i];
        if let Some(pos) = line.find('S') {
            return Some(Pos { x: i, y: pos });
        }
    }
    None
}

fn get_neighbors(lines: &Vec<&str>, index: &Pos) -> Vec<Pos> {
    let mut new_list = Vec::new();
    let cur = getchar!(lines, index);
    match cur {
        'S' => {
            if index.x >= 1 {
                let n = Pos {
                    y: index.y,
                    x: index.x - 1,
                };
                let nn = getchar!(lines, n);
                if nn == '|' || nn == '7' || nn == 'F' {
                    new_list.push(n);
                }
            }
             if index.y + 1 < lines.len() {
                let e = Pos {
                    y: index.y + 1,
                    x: index.x,
                };
                let ee = getchar!(lines, e);
                if ee == '-' || ee == 'J' || ee == '7' {
                    new_list.push(e);
                }
             }
             if index.x + 1 < lines.len() {
                let s = Pos {
                    y: index.y,
                    x: index.x + 1,
                };
                let ss = getchar!(lines, s);
                if ss == '|' || ss == 'J' || ss == 'L' {
                    new_list.push(s);
                }
             }
             if index.y >= 1 {
                let w = Pos {
                    y: index.y - 1,
                    x: index.x,
                };
                let ww = getchar!(lines, w);
                if ww == '-' || ww == 'L' || ww == 'F' {
                    new_list.push(w);
                }
             }
        },
        '|' => {
            if index.x >= 1 {
                new_list.push(Pos {
                    y: index.y,
                    x: index.x - 1,
                });
            }
            if index.x + 1 < lines.len() {
                new_list.push(Pos {
                    y: index.y,
                    x: index.x + 1,
                });
            }
        },
        '-' => {
            if index.y >= 1 {
                new_list.push(Pos {
                    y: index.y - 1,
                    x: index.x,
                });
            }
            if index.y + 1 < lines.len() {
                new_list.push(Pos {
                    y: index.y + 1,
                    x: index.x,
                });
            }
        },
        'L' => {
            if index.x >= 1 {
                new_list.push(Pos {
                    y: index.y,
                    x: index.x - 1,
                });
            }
            if index.y + 1 < lines.len() {
                new_list.push(Pos {
                    y: index.y + 1,
                    x: index.x,
                });
            }
        },
        'J' => {
            if index.x >= 1 {
                new_list.push(Pos {
                    y: index.y,
                    x: index.x - 1,
                });
            }
            if index.y >= 1 {
                new_list.push(Pos {
                    y: index.y - 1,
                    x: index.x,
                });
            }
        },
        '7' => {
            if index.x + 1 < lines.len() {
                new_list.push(Pos {
                    y: index.y,
                    x: index.x + 1,
                });
            }
            if index.y >= 1 {
                new_list.push(Pos {
                    y: index.y - 1,
                    x: index.x,
                });
            }
        },
        'F' => {
            if index.x + 1 < lines.len() {
                new_list.push(Pos {
                    y: index.y,
                    x: index.x + 1,
                });
            }
            if index.y + 1 < lines.len() {
                new_list.push(Pos {
                    y: index.y + 1,
                    x: index.x,
                });
            }
        },
        _ => (),
    };
    println!("{:?}", new_list);
    new_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(run_test("input2.txt"), 8);
    }
}

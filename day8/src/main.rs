// use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

enum Part {
    PartOne,
    PartTwo,
}

fn main() {
    run_part("input.txt", Part::PartTwo);
}

fn run_part(fname: &str, part: Part) -> usize {
    let contents = fs::read_to_string(fname).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    assert!(lines.len() > 3);

    let commands = lines[0];
    let mut nodes: HashMap<&str, Node> = HashMap::new();
    let re = Regex::new(r"^([A-Z]{3})\s=\s\(([A-Z]{3}),\s([A-Z]{3})\)").unwrap();

    for i in 2..lines.len() {
        let line = lines[i];

        if let Some(caps) = re.captures(line) {
            let name = caps.get(1).map(|m| m.as_str()).unwrap();
            let left = caps.get(2).map(|m| m.as_str()).unwrap();
            let right = caps.get(3).map(|m| m.as_str()).unwrap();
            nodes.insert(name, Node { name, left, right });
        } else {
            panic!("error during regex {} -- {}", i, line);
        }
    }

    match part {
        Part::PartOne => {
            let mut cur_node = nodes.get("AAA").unwrap();
            let mut nr_jumps = 0;
            while cur_node.name != "ZZZ" {
                for c in commands.chars() {
                    if c == 'L' {
                        cur_node = nodes.get(cur_node.left).unwrap();
                    } else {
                        cur_node = nodes.get(cur_node.right).unwrap();
                    }
                    nr_jumps += 1;
                }
            }

            println!("nr of jumps: {}", nr_jumps);
            return nr_jumps;
        },
        Part::PartTwo => {
            let keys:Vec<&'a str> = nodes.keys().filter(|k| k.ends_with('Z')).collect();

            
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(run_part("input2.txt", Part::PartOne), 2);
    }

    #[test]
    fn test_pt1_2() {
        assert_eq!(run_part("input3.txt", Part::PartOne), 6);
    }

    #[test]
    fn test_pt2_1() {
        assert_eq!(run_part("input4.txt", Part::PartTwo), 6);
    }
}

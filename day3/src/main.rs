use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").unwrap().lines().collect();
}

use std::fs;

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Map {
    Visited(usize),
    Unseen,
    Wall,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut map: Vec<Vec<Map>> = vec![vec![Map::Unseen; lines.first().unwrap().len()]; lines.len()];

    let start_index: Pos = get_start_index(lines).unwrap();

    let mut cur_index: &Pos = &start_index;
    loop {

    }

    println!("{:?}", start_index);
}

fn get_start_index(lines: Vec<&str>) -> Option<Pos> {
    for i in 0..lines.len() {
        let line = lines[i];
        if let Some(pos) = line.find('S') {
            return Some(Pos { x: i, y: pos });
        }
    }
    None
}

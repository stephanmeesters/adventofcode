use std::fs;

macro_rules! getchar{
    ($a:expr,$b:expr)=>{
        {
$a[$b.x].chars().nth($b.y).unwrap()
        }
    }
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let start_index: Pos = get_start_index(&lines).unwrap();
    println!("{:?}", start_index);

    let mut cur_index: &Pos = &start_index;
    loop {
        let cur_str = getchar!(lines, cur_index);
        println!("{}", cur_str);
        break;
    }

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

fn get_neighbors(lines: &Vec<&str>, index: Pos) -> (Pos, Pos) {
        let cur = getchar!(lines, index);
match char {
    'S' => {
        let n1: Pos;
        let n2: Pos;

    },
    '|' => return (Pos { x: index.x, y: index.y - 1} , Pos { x: index.x ,y:index.y + 1}),

}
}

fn get_chars_around(lines: &Vec<&str>, index: Pos) -> Vec<Pos> {
    let neigh:Vec<Pos> = Vec::new();
    if index.x - 1 >= 0 {
        neigh.push(Pos { x: index.x - 1, y: index.y });
    }
    if index.x + 1 < lines[0].len() {
        neigh.push(Pos { x: index.x - 1, y: index.y });
    }
    if index.y - 1 >= 0 {
        neigh.push(Pos { x: index.x, y: index.y - 1 });
    }
    if index.y + 1 < lines.len() {
        neigh.push(Pos { x: index.x, y: index.y + 1 });
    }
    neigh
}

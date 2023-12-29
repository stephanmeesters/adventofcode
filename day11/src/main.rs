use std::fs;

#[derive(Debug, PartialEq)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() {
    run("input.txt");
}

fn run(filepath: &str) {
    let contents = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_columns: Vec<usize> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if !line.contains('#') {
            empty_rows.push(i);
        }
    }
    for i in 0..lines[0].len() {
        let mut empty = true;
        for j in 0..lines.len() {
            if lines[j].chars().nth(i).unwrap() == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns.push(i);
        }
    }

    let mut galaxies: Vec<Galaxy> = Vec::new();
    for i in 0..lines[0].len() {
        let xoffset = empty_columns.iter().filter(|c| **c < i).count();
        for j in 0..lines.len() {
            if lines[j].chars().nth(i).unwrap() == '#' {
                let yoffset = empty_rows.iter().filter(|c| **c < j).count();
                galaxies.push(Galaxy {
                    x: i + xoffset*(1000000-1),
                    y: j + yoffset*(1000000-1),
                });
            }
        }
    }

    let mut total_dist: i64 = 0;
    for galaxy in galaxies.iter() {
        let mut best_dist = 0;
        for other_galaxy in galaxies.iter() {
            if galaxy == other_galaxy {
                continue;
            }
            let dx: i64 = galaxy.x as i64 - other_galaxy.x as i64;
            let dy: i64 = galaxy.y as i64 - other_galaxy.y as i64;
            let dist = dx.abs() + dy.abs();
            best_dist += dist;
        }
        total_dist += best_dist;
    }
    println!("total dist: {}", total_dist/2);

    // for j in 0..lines.len() + empty_rows.len() {
    //     for i in 0..lines[0].len() + empty_columns.len() {
    //         if let Some(galaxy) = galaxies.iter().find(|g| g.x == i && g.y == j) {
    //             print!("{}", galaxies.iter().position(|g| g == galaxy).unwrap());
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
}

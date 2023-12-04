use std::fs;
use std::cell::RefCell;

#[derive(Debug)]
struct Gear {
    line: usize,
    position: usize,
    parts: RefCell<Vec<Part>>,
}

#[derive(Debug, Copy, Clone)]
struct Part {
    value: usize,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let line_length = lines[0].len();

    let mut gears: Vec<Gear> = Vec::new();
    for i in 0..lines.len() {
        let line = lines[i];
        for (ind, char) in line.chars().enumerate() {
            if !char.is_numeric() && char != '.' {
                gears.push(Gear {
                    line: i,
                    position: ind,
                    parts: RefCell::new(Vec::new())
                });
            }
        }
    }

    let mut part_numbers: Vec<Part> = Vec::new();
    for i in 0..lines.len() {
        let line = lines[i];
        let mut left: i32 = -1;
        let mut right: i32 = -1;
        let mut around_symbol = false;
        let mut found_gears: Vec<&Gear> = Vec::new();
        for (ind, char) in line.chars().enumerate() {
            let indc = ind.try_into().unwrap();
            let char_is_numeric = char.is_numeric();
            if char_is_numeric {
                if left == -1 {
                    left = indc;
                    right = indc;
                } else {
                    right = indc;
                }
                if !around_symbol {
                    if i > 0 {
                        match scan(ind, i - 1, &gears) {
                            Some(gear) => {
                                around_symbol = true;
                                found_gears.push(gear);
                            }
                            None => (),
                        };
                    }
                    if i < line.len() - 1 {
                        match scan(ind, i + 1, &gears) {
                            Some(gear) => {
                                around_symbol = true;
                                found_gears.push(gear);
                            }
                            None => (),
                        };
                    }
                    match scan(ind, i, &gears) {
                        Some(gear) => {
                            around_symbol = true;
                            found_gears.push(gear);
                        }
                        None => (),
                    };
                }
            }

            if !char_is_numeric || ind == line_length - 1 {
                if left != -1 {
                    if around_symbol {
                        right += 1;
                        let val: usize = line[left as usize..right as usize].parse().unwrap();
                        let part = Part { value: val };
                        part_numbers.push(part);
                        for gear in &found_gears {
                            gear.parts.borrow_mut().push(part.clone());
                        }
                        found_gears.clear();
                    }
                    left = -1;
                    right = -1;
                    around_symbol = false;
                }
            }
        }
    }

    println!("{}", part_numbers.iter().map(|p| p.value).sum::<usize>());

    println!("{}", gears.iter()
             .filter(|g| g.parts.borrow().len() == 2)
             .map(|g| g.parts.borrow().iter().map(|p| p.value).product::<usize>()).sum::<usize>());
}

fn scan(ind: usize, row: usize, gears: &Vec<Gear>) -> Option<&Gear> {
    for gear in gears.into_iter().filter(|g| g.line == row) {
        if usize::abs_diff(gear.position, ind) <= 1 {
            return Some(gear);
        }
    }
    None
}

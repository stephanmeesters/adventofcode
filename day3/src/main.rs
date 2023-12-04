use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let line_length = lines[0].len();

    let mut symbols: Vec<Vec<usize>> = Vec::with_capacity(lines.len());
    for i in 0..lines.len() {
        let line = lines[i];
        let mut indices = Vec::new();
        for (ind, char) in line.chars().enumerate() {
            if !char.is_numeric() && char != '.' {
                indices.push(ind);
            }
        }
        println!("{:?}", indices);
        symbols.push(indices);
    }

    let mut part_numbers: Vec<usize> = Vec::new();
    for i in 0..lines.len() {
        let line = lines[i];
        let mut left: i32 = -1;
        let mut right: i32 = -1;
        let mut around_symbol = false;
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
                        match scan(ind, i - 1, &symbols) {
                            Some(res) => {
                                around_symbol = true;
                            }
                            None => ()
                        };
                    }
                    around_symbol |= scan(ind, i, &symbols);
                    if i < lines.len() - 1 {
                        around_symbol |= scan(ind, i+1, &symbols);
                    }
                }
            }

            if !char_is_numeric || ind == line_length - 1{
                if left != -1 {
                    if around_symbol {
                        right += 1;
                        let part:usize = line[left as usize..right as usize].parse().unwrap();
                        part_numbers.push(part);
                    }
                    left = -1;
                    right = -1;
                    around_symbol = false;
                }
            }
        }
    }

    println!("{:?} {}", part_numbers, part_numbers.iter().sum::<usize>());
}

fn scan(ind: usize, row: usize, symbols: &Vec<Vec<usize>>) -> Option<usize> {
    for &symbol in &symbols[row] {
        if usize::abs_diff(symbol, ind) <= 1 {
            return Some(ind);
        }
    }
    None
}

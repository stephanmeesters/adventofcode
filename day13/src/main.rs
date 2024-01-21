use std::fs;

fn main() {
    let result = run_part1("input.txt");
    println!("total count: {}", result);

    let result = run_part2("input.txt");
    println!("total count: {}", result);
}

#[derive(Debug)]
enum SymmetryLoc {
    None,
    Row(usize),
    Col(usize),
}

fn run_part1(input: &str) -> usize {
    let contents = fs::read_to_string(input).unwrap();

    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut total: usize = 0;
    let mut it = contents.lines().peekable();
    while let Some(line) = it.next() {
        if line.is_empty() || it.peek().is_none() {
            if matrix.is_empty() {
                continue;
            }

            let mut sym_loc: SymmetryLoc = SymmetryLoc::None;
            for row in 1..(matrix.len()) {
                let search_width = std::cmp::min(row, matrix.len() - row);
                if check_symmetry(&matrix, row, search_width) {
                    sym_loc = SymmetryLoc::Row(row);
                } 
            }
            matrix = transpose(matrix);
            for col in 1..(matrix.len()) {
                let search_width = std::cmp::min(col, matrix.len() - col);
                if check_symmetry(&matrix, col, search_width) {
                    sym_loc = SymmetryLoc::Col(col);
                } 
            }

            if let SymmetryLoc::Row(row) = sym_loc {
                total += row * 100;
            } else if let SymmetryLoc::Col(col) = sym_loc {
                total += col;
            }

            matrix.clear();
        } else {
            matrix.push(line.chars().map(|c| c == '.').collect());
        }
    }

    total
}

fn run_part2(input: &str) -> usize {
    let contents = fs::read_to_string(input).unwrap();

    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut total: usize = 0;
    let mut it = contents.lines().peekable();
    while let Some(line) = it.next() {
        if line.is_empty() || it.peek().is_none() {
            if matrix.is_empty() {
                continue;
            }

            let mut sym_loc: SymmetryLoc = SymmetryLoc::None;
            let mut sym_loc_smudge: SymmetryLoc = SymmetryLoc::None;
            for row in 1..(matrix.len()) {
                let search_width = std::cmp::min(row, matrix.len() - row);
                if check_symmetry(&matrix, row, search_width) {
                    sym_loc = SymmetryLoc::Row(row);
                } 
            }
            for row in 1..(matrix.len()) {
                let search_width = std::cmp::min(row, matrix.len() - row);
                if check_symmetry_smudge(&matrix, row, search_width){
                    if let SymmetryLoc::Row(prow) = sym_loc {
                        if prow != row {
                            sym_loc_smudge = SymmetryLoc::Row(row);
                        }
                    }
                    else {
                        sym_loc_smudge = SymmetryLoc::Row(row);
                    }
                } 
            }
            matrix = transpose(matrix);
            for col in 1..(matrix.len()) {
                let search_width = std::cmp::min(col, matrix.len() - col);
                if check_symmetry(&matrix, col, search_width) {
                    sym_loc = SymmetryLoc::Col(col);
                } 
            }
            for col in 1..(matrix.len()) {
                let search_width = std::cmp::min(col, matrix.len() - col);
                if check_symmetry_smudge(&matrix, col, search_width){
                    if let SymmetryLoc::Col(pcol) = sym_loc {
                        if pcol != col {
                            sym_loc_smudge = SymmetryLoc::Col(col);
                        }
                    }
                    else {
                        sym_loc_smudge = SymmetryLoc::Col(col);
                    }
                } 
            }

            if let SymmetryLoc::Row(row) = sym_loc_smudge {
                total += row * 100;
            } else if let SymmetryLoc::Col(col) = sym_loc_smudge {
                total += col;
            }

            matrix.clear();
        } else {
            matrix.push(line.chars().map(|c| c == '.').collect());
        }
    }

    total
}

fn check_symmetry(matrix: &Vec<Vec<bool>>, row: usize, size: usize) -> bool {
    let result: usize = matrix[row - size..row]
        .iter()
        .zip(matrix[row..row + size].iter().rev())
        .map(|(a, b)| process_cols(a, b))
        .sum();
    result == 0
}

fn check_symmetry_smudge(matrix: &Vec<Vec<bool>>, row: usize, size: usize) -> bool {
    let result: usize = matrix[row - size..row]
        .iter()
        .zip(matrix[row..row + size].iter().rev())
        .map(|(a, b)| process_cols(a, b))
        .sum();
    result == 1
}

fn process_cols(a: &Vec<bool>, b: &Vec<bool>) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(&a, &b)| (a ^ b) as usize)
        .sum()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(run_part1("input2.txt"), 405);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(run_part2("input2.txt"), 400);
    }

    #[test]
    fn test_pt1_2() {
        assert_eq!(run_part1("input3.txt"), 709);
    }

    #[test]
    fn test_pt1_3() {
        assert_eq!(run_part1("input4.txt"), 11);
    }

    #[test]
    fn test_pt1_4() {
        assert_eq!(run_part1("input5.txt"), 3);
    }
}

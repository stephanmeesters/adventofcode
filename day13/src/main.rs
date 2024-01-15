use std::fs;

fn main() {
    let result = run("input.txt");
    println!("total count: {}", result);
}

fn run(input: &str) -> usize {
    let contents = fs::read_to_string(input).unwrap();

    let mut matrix: Vec<Vec<bool>> = Vec::new();
    // let mut total: usize = 0;
    let mut it = contents.lines().peekable();
    while let Some(line) = it.next() {
        if line.is_empty() || it.peek().is_none() {
            let mut count = 0;
            let search_width = 1;

            for row in search_width..(matrix.len() - search_width) {
                let bb = check_symmetry(&matrix, row, search_width);
                if bb {
                    count +=1 ;
                }
                // println!("{} {}", bb, row);
            }
            matrix = transpose(matrix);
            for row in search_width..(matrix.len() - search_width) {
                let bb = check_symmetry(&matrix, row, search_width);
                if bb {
                    count +=1 ;
                }
                // println!("{} {}", bb, row);
            }
            println!("found {}", count);
            matrix.clear();
        } else {
            matrix.push(line.chars().map(|c| c == '.').collect());
        }
    }

    0
    // total
}

fn check_symmetry(matrix: &Vec<Vec<bool>>, row: usize, size: usize) -> bool {
    // println!("{}", row + size);
    let result: usize = matrix[row - size..row]
        .iter()
        .zip(matrix[row..row + size].iter().rev())
        .map(|(a, b)| process_cols(a, b))
        .sum();
    result == 0
}

fn process_cols(a: &Vec<bool>, b: &Vec<bool>) -> usize {
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("----");
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

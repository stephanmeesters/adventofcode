use std::fs;

fn main() {
    let contents = fs::read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    for line in lines {
        let inputs: Vec<&str> = line.split(' ').collect();
        assert_eq!(inputs.len(), 2);
        let left = inputs[0];
        let right = inputs[1];

        let groups: Vec<&str> = left.split('.').filter(|k| !k.is_empty()).collect();
        println!("{:?}", groups);
    }
}

fn calc_permutations(input: &str, right: &[usize]) -> u32 {
    let total_springs = right.iter().sum::<usize>();
    let num_dots = right.len() - 1;
    let tlen = total_springs + num_dots;
    if input.len() < tlen {
        return 0;
    }
    let num_movable = input.len() - tlen;

    let num_to_add = total_springs - input.chars().filter(|&c| c == '#').count();
    let num_unknowns = input.chars().filter(|&c| c == '?').count();
    if num_to_add < num_unknowns {
        return 0;
    }

    let dp_table:Vec<Vec<usize>> = vec![vec![]];
    for i in 0..right.len() {
        let it = right.iter().skip(i);
        let right = it.sum() + it.len();
    }


    ??#??    add: 3   unknowns: 4   numdots: 1


    ####.
    ###.#
    #.###
    .####
}

fn check(refstring:&str, offset:usize, width:usize, right:usize, checkall:bool) -> bool {
    if offset + width + right > refstring.len() {
        return false;
    }
    return refstring.chars().enumerate().take(offset + width).any(|(i, c)| {
        if i < offset || i > offset + width {
            return c == '.';
        }
        return c == '#';
    })
}









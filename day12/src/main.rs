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
    let tlen = right.iter().sum::<usize>() + right.len() - 1;
    if input.len() < tlen {
        return 0;
    }
    let str = input.replace("#", "");
    if str.len() <= 1 {
        return 1;
    }
    0
}

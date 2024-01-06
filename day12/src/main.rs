use std::fs;

fn main() {
    let result = run("input.txt");
    println!("total count: {}", result);
}

fn run(input:&str) -> i32 {
    let contents = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut total = 0;
    for line in lines {
        let inputs: Vec<&str> = line.split(' ').collect();
        assert_eq!(inputs.len(), 2);

        let left = inputs[0];
        let right:Vec<usize> = inputs[1].split(',').map(|k| k.parse::<usize>().unwrap()).collect();

        let outcome = calc_permutations(left, &right);
        total += outcome;
    }

    total
}

fn calc_permutations(input: &str, right: &[usize]) -> i32 {
    let total_springs = right.iter().sum::<usize>();
    let num_dots = right.len() - 1;
    let tlen = total_springs + num_dots;
    if input.len() < tlen {
        return 0;
    }

    let free_dots = input.len() - tlen;
    if free_dots == 0 {
        return 1;
    }

    let mut groups_owned: Vec<Vec<String>> = Vec::new();
    for g in 0..right.len() {
        let mut group:Vec<String> = Vec::new();
        let first_dot = if g == 0 { 0} else {1};
        let n_hashes = right[g];
        for ii in 0..free_dots+1 {
            let mut str: String = String::from_utf8(vec![b'.'; ii+first_dot]).unwrap();
            let str2: String = String::from_utf8(vec![b'#'; n_hashes]).unwrap();
            str.push_str(&str2);
            group.push(str);
        }
        groups_owned.push(group);
    }
    let groups: Vec<Vec<&str>> = groups_owned.iter()
        .map(|inner_vec| inner_vec.iter().map(|s| s.as_str()).collect())
        .collect();
    // println!("{:?}", groups);

    recurse(input, &groups)
}

fn recurse(str: &str, groups: &[Vec<&str>]) -> i32 {
    // println!("str: {} -- groups: {}", str, groups.len());
    if str.len() == 0 || groups.len() == 0 {
        return 1;
    }

    let mut total = 0;
    let group = groups.first().unwrap();
    for single in group.iter() {
        if string_ok(str, single) {
            let substr = &str[single.len()..];
            total += recurse(substr, &groups[1..]);
        }
    }

    total
}

fn string_ok(refstr: &str, group: &str) -> bool {
    if group.len() > refstr.len() {
        return false;
    }

    for i in 0..std::cmp::min(refstr.len(), group.len()) {
        let refchar = refstr.chars().nth(i).unwrap();
        let groupchar = group.chars().nth(i).unwrap();
        if refchar != '?' && groupchar != refchar {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurse() {
        let start = "?#???";
        let groups = vec![vec!["##", ".##"], vec![".#", "..#"]];
        let outcome = recurse(start, &groups);
        assert_eq!(outcome, 3);
    }

    #[test]
    fn test_calc_permutations() {
        let start = "?#???";
        let groups:Vec<usize> = vec![2, 1];
        let outcome = calc_permutations(start, &groups);
        assert_eq!(outcome, 3);
    }

    #[test]
    fn run_on_test_input() {
        assert_eq!(run("input2.txt"), 21);
    }
}

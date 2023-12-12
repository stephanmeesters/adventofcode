use regex::Regex;
use std::fs;

fn main() {
    println!("result {}", run("input.txt"));
}

fn run(input: &str) -> i64 {
    let contents = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let re = Regex::new(r"\d+").unwrap();

    let mut total = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        let numbers: Vec<i64> = re
            .find_iter(line)
            .map(|f| f.as_str().parse::<i64>().unwrap())
            .collect();
        let mut diff_list: Vec<Vec<i64>> = Vec::new();

        let mut curr: &Vec<i64> = &numbers;
        loop {
            let diff: Vec<i64> = differences(curr);
            if diff.iter().sum::<i64>() == 0 || diff.len() < 2 {
                break;
            }
            diff_list.push(diff);
            curr = &(diff_list.last().unwrap());
        }

        let mut next = 0;
        for i in (0..diff_list.len()).rev() {
            next = diff_list[i].last().unwrap() + next;
        }
        total += numbers.last().unwrap() + next;
    }

    total
}

fn differences(input: &Vec<i64>) -> Vec<i64> {
    assert!(input.len() > 1);
    let mut new_arr = Vec::with_capacity(input.len() - 1);
    for i in 0..input.len() - 1 {
        new_arr.push(input[i + 1] - input[i]);
    }
    new_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_1() {
        assert_eq!(run("input2.txt"), 114);
    }
}

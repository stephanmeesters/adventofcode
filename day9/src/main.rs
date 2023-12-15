// use regex::Regex;
// use std::fs;
//
// fn main() {
//     println!("result {}", run("input.txt"));
// }
//
// fn run(input: &str) -> i64 {
//     let contents = fs::read_to_string(input).unwrap();
//     let lines: Vec<&str> = contents.lines().collect();
//     let re = Regex::new(r"\d+").unwrap();
//
//     let mut total = 0;
//     for i in 0..lines.len() {
//         let line = lines[i];
//         let numbers: Vec<i64> = re
//             .find_iter(line)
//             .map(|f| f.as_str().parse::<i64>().unwrap())
//             .collect();
//         let mut diff_list: Vec<Vec<i64>> = Vec::new();
//
//         let mut curr: &Vec<i64> = &numbers;
//         loop {
//             let diff: Vec<i64> = differences(curr);
//             if diff.iter().sum::<i64>() == 0 || diff.len() < 2 {
//                 break;
//             }
//             diff_list.push(diff);
//             curr = &(diff_list.last().unwrap());
//         }
//
//         let mut next = 0;
//         for i in (0..diff_list.len()).rev() {
//             next = diff_list[i].last().unwrap() + next;
//         }
//         total += numbers.last().unwrap() + next;
//
//         println!("\n\n{:?}\n\n {:?}\n\n {}", numbers, diff_list, numbers.last().unwrap() + next);
//     }
//
//     total
// }
//
// fn differences(input: &Vec<i64>) -> Vec<i64> {
//     assert!(input.len() > 1);
//     let mut new_arr = Vec::with_capacity(input.len() - 1);
//     for i in 0..input.len() - 1 {
//         new_arr.push(input[i + 1] - input[i]);
//     }
//     new_arr
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_pt1_1() {
//         assert_eq!(run("input2.txt"), 114);
//     }
//
//     #[test]
//     fn test_pt1_2() {
//         assert_eq!(run("input3.txt"), 114);
//     }
// }
//


// use std::time::Instant;
//
// macro_rules! sequence_differences {
//     ($vec:expr) => {{
//         let mut differences = Vec::new();
//         if $vec.len() > 1 {
//             for i in 1..$vec.len() {
//                 differences.push($vec[i] - $vec[i - 1]); //we skip 0 so this is fine
//             }
//         }
//         differences
//     }};
// }
// fn main() {
//     let start = Instant::now();
//     let mut sum: i32 = 0;
//     include_str!("../input.txt")
//         .lines()
//         .into_iter()
//         .map(|l| {
//             l.split_whitespace()
//                 .into_iter()
//                 .map(|n| n.parse::<i32>().unwrap())
//                 .collect::<Vec<i32>>()
//         })
//         .into_iter()
//         .for_each(|vec: Vec<i32>| {
//             //part1
//             let nxt = *vec.last().unwrap() + calc_right(vec);
//             //part2
//             // let nxt = *vec.first().unwrap() - calc_left(vec);
//             sum += nxt;
//         });
//
//     println!("result {sum}");
//     println!("Elapsed : {:?}",start.elapsed());
// }
// fn calc_right(mut v: Vec<i32>) -> i32 {
//     let mut result: Vec<i32> = Vec::new();
//     while v.windows(2).any(|w| w[1] - w[0] != 0) {//222 || 000 
//         v = sequence_differences!(v);
//         let last = v.iter().last().unwrap();
//         result.push(*last);
//     }
//     result.into_iter().sum()
// }
// fn calc_left(mut v: Vec<i32>) -> i32 {
//     let mut result: Vec<i32> = Vec::new();
//     while v.windows(2).any(|w| w[1] - w[0] != 0) {
//         v = sequence_differences!(v);
//         let num = v[0];
//         result.push(num);
//     }
//     result.iter().rev().fold(0, |acc, x| x - acc)
// }
//
mod converter; 
use crate::converter::Parser;

fn main()
{
    solve();
}

pub fn solve() -> (i64, i64)
{
    let input = include_str!("../input3.txt").to_vec_of_vec::<i64>("\n", " ");

    println!("{:?}", input.iter().map(|v| predict(v.to_vec())));
    let result1 = input.iter().map(|v| predict(v.to_vec())).sum();
    let result2 = input.iter().map(|v| predict(v.iter().rev().copied().collect())).sum();

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn predict(mut diffs: Vec<i64>) -> i64
{
    let mut prediction = 0;

    while diffs.iter().any(|&d| d != 0) {
        prediction += update_diffs(&mut diffs);
    }

    prediction
}

fn update_diffs(values: &mut Vec<i64>) -> i64
{
    for i in 0..values.len() - 1 {
        values[i] = values[i + 1] - values[i];
    }

    values.pop().unwrap()
}

#[test]
fn test()
{
    assert_eq!(solve(), (1884768153, 1031));
}

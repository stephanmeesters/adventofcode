use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = contents.lines().map(|l| l.replace(" ","")).collect();

    let re = Regex::new(r"\d+").unwrap();

    let times: Vec<usize> = re
        .find_iter(&lines[0])
        .map(|v| v.as_str().parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = re
        .find_iter(&lines[1])
        .map(|v| v.as_str().parse::<usize>().unwrap())
        .collect();

    let mut beats_list:Vec<usize> = Vec::new();
    for (time, max_distance) in times.iter().zip(distances.iter()) {
        println!("{} {}", time, max_distance);
        let mut beats = 0;
        for time_held in 1..time - 1 {
            let remaining_time = time - time_held;
            let distance = remaining_time * time_held;
            if distance > *max_distance {
                beats += 1;
            }
        }
        beats_list.push(beats);
    }

    println!("{}", beats_list.iter().product::<usize>());
}

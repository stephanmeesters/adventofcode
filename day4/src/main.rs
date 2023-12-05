use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"\d+").unwrap();

    let contents = fs::read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut points: Vec<usize> = vec![0; lines.len()];
    let mut num_cards: Vec<usize> = vec![0; lines.len()];

    for ind in 0..lines.len() {
        let line = lines[ind];
        let parts: Vec<&str> = line.split("|").collect();
        assert!(parts.len() == 2);

        let winning_part = parts[0].split(":").last().unwrap();
        let our_part = parts[1];

        let winning_numbers: Vec<usize> = re
            .find_iter(winning_part)
            .filter_map(|d| d.as_str().parse::<usize>().ok())
            .collect();
        let our_numbers: Vec<usize> = re
            .find_iter(our_part)
            .filter_map(|d| d.as_str().parse::<usize>().ok())
            .collect();

        let mut score: usize = 0;
        for num in &our_numbers {
            if winning_numbers.contains(num) {
                score += 1;
            }
        }
        points[ind] = score;
        for oind in ind..(ind + score + 1).min(lines.len()) {
            if oind == ind && num_cards[ind] == 0 {
                num_cards[ind] = 1;
                continue;
            }
            num_cards[oind] += num_cards[ind];
        }
    }

    println!("{:?}", num_cards);

    let mut total_score = 0;
    for ind in 0..lines.len() {
        total_score += num_cards[ind];
    }

    println!("total score: {}", total_score);
}

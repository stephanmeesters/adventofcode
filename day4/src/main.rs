use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"\d+").unwrap();

    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut num_cards: Vec<usize> = vec![0; lines.len()];
    let mut total_score = 0;

    for ind in 0..lines.len() {
        let line = lines[ind];
        let parts: Vec<&str> = line.split("|").collect();
        assert_eq!(parts.len(), 2);

        let winning_part = parts[0].split(":").last().unwrap();
        let our_part = parts[1];

        let winning_numbers: Vec<usize> = re
            .find_iter(winning_part)
            .map(|d| d.as_str().parse::<usize>().unwrap())
            .collect();
        let our_numbers: Vec<usize> = re
            .find_iter(our_part)
            .map(|d| d.as_str().parse::<usize>().unwrap())
            .collect();

        let mut num_matches: usize = 0;
        let mut score: usize = 0;
        for num in &our_numbers {
            if winning_numbers.contains(num) {
                num_matches += 1;
                if score == 0 {
                    score = 1;
                } else {
                    score = score + score;
                }
            }
        }

        for oind in ind..(ind + num_matches + 1).min(lines.len()) {
            if oind == ind {
                num_cards[ind] += 1;
                continue;
            }
            num_cards[oind] += num_cards[ind];
        }

        total_score += score;
    }

    let total_num_cards: usize = num_cards.iter().sum();

    println!("total score: {}", total_score);
    println!("total num cards: {}", total_num_cards);
}

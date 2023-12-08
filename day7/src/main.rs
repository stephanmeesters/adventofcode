use std::cmp::Ordering;
use std::{collections::HashMap, fs};

static CARD_VALUES: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug)]
struct Hand<'a> {
    orig_str: &'a str,
    cards: HashMap<char, (usize, usize)>,
    bid: usize,
}

impl<'a> Hand<'a> {
    fn new(card_str: &'a str, bid: usize) -> Self {
        let mut cards = HashMap::new();
        for char in card_str.chars() {
            cards
                .entry(char)
                .and_modify(|t: &mut (usize, usize)| t.0 = t.0 + 1)
                .or_insert((1, 13 - CARD_VALUES.iter().position(|r| r == &char).unwrap()));
        }
        Self {
            orig_str: card_str,
            cards,
            bid,
        }
    }

    fn compare(a: &Hand, b: &Hand) -> Ordering {
        let a_values = a.get_sorted_values();
        let b_values = b.get_sorted_values();
        println!("{:?}", a_values);
        println!("{:?}", b_values);
        for i in 0..std::cmp::min(a_values.len(), b_values.len()) {
            let aa = a_values[i];
            let bb = b_values[i];
            println!(
                "{} {} {} -- {} {} {}",
                a.orig_str, aa.0, aa.1, b.orig_str, bb.0, bb.1
            );
            let first_cmp = aa.0.cmp(&bb.0);
            if first_cmp != std::cmp::Ordering::Equal {
                println!("first cmp {:?}", first_cmp);
                return first_cmp;
            }
        }
        for i in 0..std::cmp::min(a_values.len(), b_values.len()) {
            let aa = a_values[i];
            let bb = b_values[i];
            let snd_cmp = aa.1.cmp(&bb.1);
            if snd_cmp != std::cmp::Ordering::Equal {
                println!("snd cmp{:?}", snd_cmp);
                return snd_cmp;
            }
        }
        Ordering::Equal
    }

    fn get_sorted_values(&self) -> Vec<(usize, usize)> {
        let mut values: Vec<(usize, usize)> = self.cards.values().cloned().collect();
        values.sort_by(|a, b| {
            let first_cmp = b.0.cmp(&a.0);
            if first_cmp == std::cmp::Ordering::Equal {
                b.1.cmp(&a.1)
            } else {
                first_cmp
            }
        });
        values
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

fn main() {
    run("input.txt");
}

pub fn run(fname: &str) -> usize {
    let contents = fs::read_to_string(fname).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        assert_eq!(parts.len(), 2);
        let card_str = parts[0];
        let bid: usize = parts[1].parse().unwrap();
        let hand = Hand::new(card_str, bid);

        match hands.iter_mut().find(|h| **h == hand) {
            Some(h) => h.bid += hand.bid,
            None => hands.push(hand) 
        }
    }

    hands.dedup();
    hands.sort_by(|a, b| Hand::<'_>::compare(a, b));
    for hand in hands.iter() {
        println!("{} {:?}", hand.orig_str, hand.cards);
    }

    let total_winnings: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();
    println!("Total winnings: {}", total_winnings);

    return total_winnings;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(run("input2.txt"), 6440);
    }

    #[test]
    fn test_pt1_2() {
        assert_eq!(run("input4.txt"), 102);
    }

    #[test]
    fn test_pt1_4() {
        assert_eq!(run("input5.txt"), 47);
    }

    #[test]
    fn test_pt1_3() {
        assert_eq!(run("input3.txt"), 6592);
    }

}

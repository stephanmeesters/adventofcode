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
                .or_insert((
                    1,
                    13 - CARD_VALUES.iter().position(|r| r == &char).unwrap(),
                ));
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
        for i in 0..std::cmp::min(a_values.len(), b_values.len()) {
            let a = a_values[i];
            let b = b_values[i];
            let first_cmp = a.0.cmp(&b.0);
            if first_cmp == std::cmp::Ordering::Equal {
                return a.1.cmp(&b.1);
            } else {
                return first_cmp;
            }
        }
        Ordering::Equal
    }

    fn get_sorted_values(&self) -> Vec<(usize, usize)> {
        let mut values: Vec<(usize, usize)> = self.cards.values().cloned().collect();
        values.sort_by(|a, b| {
            let first_cmp = b.0.cmp(&a.0);
            if first_cmp == std::cmp::Ordering::Equal {
                a.1.cmp(&b.1)
            } else {
                first_cmp
            }
        });
        values
    }
}

fn main() {
    let contents = fs::read_to_string("input4.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        assert_eq!(parts.len(), 2);
        let card_str = parts[0];
        let bid: usize = parts[1].parse().unwrap();
        hands.push(Hand::new(card_str, bid));
    }

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
}

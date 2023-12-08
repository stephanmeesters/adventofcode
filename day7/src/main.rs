// use std::fmt;
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
            cards.entry(char).and_modify(|(c, _v)| *c += 1).or_insert((
                0,
                CARD_VALUES.iter().rev().position(|r| r == &char).unwrap(),
            ));
        }
        Self {
            orig_str: card_str,
            cards,
            bid,
        }
    }

    // fn value(&self) -> usize {
    //     self.cards.values().max().unwrap() * 1000000
    //         + self
    //             .cards
    //             .iter()
    //             .filter(|&(_key, &v)| v > 1)
    //             .map(|(k, v)| 1000 * v * (13 - CARD_VALUES.iter().position(|r| r == k).unwrap()))
    //             .sum::<usize>()
    // }

    fn compare(a: &Hand, b: &Hand) -> Ordering {
        // more of a kind beats less of a kind
        let a_max = a.cards.values().max().unwrap();
        let b_max = b.cards.values().max().unwrap();
        if a_max > b_max {
            return Ordering::Greater;
        }

        let a_values = a.get_sorted_values();
        let b_values = b.get_sorted_values();

        Ordering::Equal
    }

    fn get_sorted_values(&self) -> Vec<(usize, usize)> {
        let mut values: Vec<(usize, usize)> = self.cards.values().cloned().collect();
        values.sort_by(|a, b| {
            let first_cmp = a.0.cmp(&b.0);
            if first_cmp == std::cmp::Ordering::Equal {
                a.1.cmp(&b.1)
            } else {
                first_cmp
            }
        });
        values
    }
}

// impl<'a> fmt::Debug for Hand<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("Hand")
//             .field("orig_str", &self.orig_str)
//             .field("cards", &self.cards)
//             .field("bid", &self.bid)
//             .field("value", &self.value())
//             .finish()
//     }
// }

fn main() {
    let contents = fs::read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let card_str = parts[0];
        let bid: usize = parts[1].parse().unwrap();
        hands.push(Hand::new(card_str, bid));
    }

    hands.sort_by(|a, b| Hand::<'_>::compare(a, b));
    // hands.sort_by_key(|h| h.value());

    println!("{:?}", hands);
}

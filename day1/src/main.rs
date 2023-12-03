// use regex::Regex;
use std::fs;

static NUMBERS: Vec<(&str, &str)> = 
    vec!(("one", "1"), 
         ("two", "2"),
         ("three", "3"),
         ("four", "4"),
         ("five", "5"),
         ("six", "6"),
         ("seven", "7"),
         ("eight", "8"),
         ("nine", "9"));

trait StringExt {
    fn get_digits(&self) -> String;
    fn words_to_digits(&self) -> String;
}

impl StringExt for &str {
    fn get_digits(&self) -> String {
       self.to_string()
           .chars()
           .filter(|x| x.is_digit(10))
           .collect()
    }

    fn words_to_digits(&self) -> String {
        let ind = 0;
        let mut new_string = self.to_string();
        while ind < self.len() - 1 {
            let slice = &self[ind..self.len()];
            for &comp in NUMBERS {
                if slice.starts_with(&comp.0) {
                    new_string.replace_range(ind..ind+comp.0.len(), &comp.1)
                }
            }
        }
        new_string
    }
}

fn main() {
    // let re = Regex::new(r"\d+").unwrap();
    let mut total = 0;
    let mut k = 0;
    for line in fs::read_to_string("input3.txt").unwrap().lines() {
        let filtered_line = line.words_to_digits();
        // let filtered_line = convert_to_digits(line);
        let mut number = filtered_line.as_str().get_digits();
        let bbb = number.to_string();
        // let digits: Vec<&str> = re.find_iter(&filtered_line).map(|c| c.as_str()).collect();
        // let mut number = digits.join("");
        if number.len() > 2 {
            number.drain(1..number.len() - 1);
        }
        let parsed: i64 = number.parse().unwrap();

        if number.len() == 1 {
            total += parsed * 10 + parsed;
        }
        else {
            total += parsed;
        }

        println!("{} {} -- {} -- {} -- {}", k, line, filtered_line, bbb, number);
        k += 1;
    }
    println!("Total number: {}", total);
}

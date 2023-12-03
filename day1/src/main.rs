use std::fs;

static NUMBERS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

trait StringExt {
    fn get_digits(&self) -> String;
    fn words_to_digits(&self) -> String;
}

impl StringExt for &str {
    fn get_digits(&self) -> String {
        self.chars().filter(|x| x.is_digit(10)).collect()
    }

    fn words_to_digits(&self) -> String {
        let mut ind = 0;
        let mut new_string = self.to_string();
        while ind < new_string.len() {
            let slice = &new_string[ind..];
            for &(word, number) in &NUMBERS {
                if slice.starts_with(word) {
                    new_string.replace_range(ind..ind + word.len(), number);
                    break;
                }
            }
            ind += 1;
        }
        new_string
    }
}


fn main() {
    let mut total = 0;
    let mut k = 0;
    for line in fs::read_to_string("input3.txt").unwrap().lines() {
        let filtered_line = line.words_to_digits();
        let mut number = filtered_line.as_str().get_digits();
        let bbb = number.to_string();
        if number.len() > 2 {
            number.drain(1..number.len() - 1);
        }
        let parsed: i64 = number.parse().unwrap();

        if number.len() == 1 {
            total += parsed * 10 + parsed;
        } else {
            total += parsed;
        }

        println!(
            "{} {} -- {}",
            k, line, number
        );
        k += 1;
    }
    println!("Total number: {}", total);
}

use regex::Regex;
use std::fs;

fn main() {
    let num_red = 12;
    let num_green = 13;
    let num_blue = 14;

    let re_game = Regex::new(r"Game\s(?P<game>\d+)").unwrap();
    let re_red = Regex::new(r"(?P<red>\d+)\s*red").unwrap();
    let re_green = Regex::new(r"(?P<green>\d+)\s*green").unwrap();
    let re_blue = Regex::new(r"(?P<blue>\d+)\s*blue").unwrap();

    let mut total = 0;
    let mut power_sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut success = true;
        let mut game:i32 = -1;
        let parts = line.split(";");
        let mut power = 0;

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for part in parts {
            if game == -1 {
                game = match re_game.captures(part) {
                    Some(caps) => caps.name("game").map_or("0", |m| m.as_str()).parse().unwrap(),
                    None => 0
                };
            }
            let red:i32 = match re_red.captures(part) {
                Some(caps) => caps.name("red").map_or("0", |m| m.as_str()).parse().unwrap(),
                None => 0
            };
            let green:i32 = match re_green.captures(part) {
                Some(caps) => caps.name("green").map_or("0", |m| m.as_str()).parse().unwrap(),
                None => 0
            };
            let blue:i32 = match re_blue.captures(part) {
                Some(caps) => caps.name("blue").map_or("0", |m| m.as_str()).parse().unwrap(),
                None => 0
            };
            if red > num_red || green > num_green || blue > num_blue {
                success = false;
            }

            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }

        let loc_power = max_red * max_green * max_blue;
        if loc_power > power {
            power = loc_power;
        }

        if success {
            total += game;
        }
        power_sum += power;
    }
    println!("total: {} powersum: {}", total, power_sum);
}

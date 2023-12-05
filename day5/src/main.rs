use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::{fs, ops::Range};

#[derive(Debug)]
struct Map {
    source_ranges: RefCell<Vec<Range<usize>>>,
    destination_ranges: RefCell<Vec<Range<usize>>>,
    map_type: MapType,
}

impl Map {
    fn transfer(&self, nr: usize) -> usize {
        let source_ranges = self.source_ranges.borrow();
        let destination_ranges = self.destination_ranges.borrow();
        for (ind, range) in source_ranges.iter().enumerate() {
            if range.contains(&nr) {
                return destination_ranges[ind].start + (nr - range.start);
            }
        }
        return nr;
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum MapType {
    None,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let seeds = get_numbers_from_line(lines[0]);

    let mut converters: Vec<Map> = Vec::new();

    let mut cur_scan_type: MapType = MapType::None;
    for i in 1..lines.len() {
        let line = lines[i];
        if line.is_empty() {
            continue;
        }

        match line {
            "seed-to-soil map:" => cur_scan_type = MapType::SeedToSoil,
            "soil-to-fertilizer map:" => cur_scan_type = MapType::SoilToFertilizer,
            "fertilizer-to-water map:" => cur_scan_type = MapType::FertilizerToWater,
            "water-to-light map:" => cur_scan_type = MapType::WaterToLight,
            "light-to-temperature map:" => cur_scan_type = MapType::LightToTemperature,
            "temperature-to-humidity map:" => cur_scan_type = MapType::TemperatureToHumidity,
            "humidity-to-location map:" => cur_scan_type = MapType::HumidityToLocation,
            _ => {
                let map = converters.last().unwrap();
                let (source_range, destination_range) = get_ranges(line);
                map.source_ranges.borrow_mut().push(source_range);
                map.destination_ranges.borrow_mut().push(destination_range);
            }
        }

        if converters.is_empty() || converters.last().unwrap().map_type != cur_scan_type {
            converters.push(Map {
                source_ranges: RefCell::new(vec![]),
                destination_ranges: RefCell::new(vec![]),
                map_type: cur_scan_type,
            });
        }
    }

    let mut min_num = usize::max_value();
    for seed in seeds.iter() {
        let mut nr = *seed;
        for converter in converters.iter() {
            nr = converter.transfer(nr);
        }
        if nr < min_num {
            min_num = nr;
        }
    }
    println!("{}", min_num); 

    min_num = usize::max_value();
    for (start, len) in seeds.iter().tuples() {
        for seed in *start..*start + *len {
            let mut nr = seed;
            for converter in converters.iter() {
                nr = converter.transfer(nr);
            }
            if nr < min_num {
                min_num = nr;
            }
        }
    }
    println!("{}", min_num); 
}

// outputs (source range, destination range)
fn get_ranges(line: &str) -> (Range<usize>, Range<usize>) {
    let numbers = get_numbers_from_line(line);
    assert_eq!(numbers.len(), 3);
    (
        numbers[1]..numbers[1] + numbers[2],
        numbers[0]..numbers[0] + numbers[2],
    )
}

fn get_numbers_from_line(line: &str) -> Vec<usize> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(line)
        .map(|d| d.as_str().parse::<usize>().unwrap())
        .collect()
}

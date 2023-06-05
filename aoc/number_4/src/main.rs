use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Pair {
    first: i32,
    last: i32,
}

impl Pair {
    fn new(first: i32, last: i32) -> Self {
        // struct constructor
        Self{
            first, 
            last,
        }
    }
   
    fn contains(&self, other: &Pair) -> bool {
        self.first <= other.first && self.last >= other.last
    }

    fn overlaps(&self, other: &Pair) -> bool {
        (self.first >= other.first && self.first <= other.last) || (self.last >= other.first && self.last <= other.last)
    }
}

fn read_file(filename: String) -> Vec<(Pair, Pair)> {

    let mut data: Vec::<(Pair, Pair)> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let pairs = line.unwrap().clone();
        let parts = pairs.split(",").collect::<Vec<&str>>();
        
        let string_pair_1 = parts[0].split("-").collect::<Vec<&str>>();
        let first_1: i32 = string_pair_1[0].trim().parse().unwrap();
        let last_1: i32 = string_pair_1[1].trim().parse().unwrap();
        let pair_1 = Pair::new(first_1, last_1);

        let string_pair_2 = parts[1].split("-").collect::<Vec<&str>>();
        let first_2: i32 = string_pair_2[0].trim().parse().unwrap();
        let last_2: i32 = string_pair_2[1].trim().parse().unwrap();
        let pair_2 = Pair::new(first_2, last_2);
        
        data.push((pair_1, pair_2));
    }
    data
}


fn main() {
    let data = read_file("input_data.txt".to_string());
    
    let mut contained_intervals = 0;
    for (p1, p2) in &data {
        if p1.contains(&p2) || p2.contains(&p1) {
            contained_intervals += 1;
        }
    }
    println!("Contained intervals: {}", contained_intervals);

    let mut overlaping_intervals = 0;
    for (p1, p2) in &data {
        if p1.overlaps(&p2) || p2.overlaps(&p1) {
            overlaping_intervals += 1;
        }
    }
    println!("Overlaping intervals: {}", overlaping_intervals);

}

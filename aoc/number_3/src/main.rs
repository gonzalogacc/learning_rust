//https://adventofcode.com/2022/day/3

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;


fn read_file(filename: String) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        content.push(line.unwrap());
    }
    content
}

fn process_rucksack(rs: &String) -> i32 {
    // Split the rucksack in half

    let vrs: Vec<char> = rs.chars().collect();
    
    let midpoint = vrs.len()/2;

    let mut first_comp: HashSet<char> = HashSet::new();
    let mut second_comp: HashSet<char> = HashSet::new();
    
    for i in 0..midpoint {
        first_comp.insert(vrs[i]);
        second_comp.insert(vrs[i+midpoint]);
    }
    
    let value_pos = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut total_value = 0;
    for intersection in first_comp.intersection(&second_comp){
        let value = value_pos.chars().position(|c| c == *intersection).unwrap() as i32;
        total_value += value+1;
        println!("{}: {}", intersection, value);
    }
    total_value
}

fn main() {
    let filename = "input_data.txt".to_string();
    let file_content: Vec<String> = read_file(filename);
    
    let mut total_prio = 0;
    for line in &file_content {
        total_prio += process_rucksack(line);
    };
    println!("Priority total of items: {total_prio}");
}

//https://adventofcode.com/2022/day/3

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};


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
    
    let mut total_value = 0;
    for intersection in first_comp.intersection(&second_comp){
        let value = item_value(intersection);
        total_value += value;
    }
    total_value
}

fn item_value(vc: &char) -> i32 {
    let value_pos = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let value = value_pos.chars().position(|c| c == *vc).unwrap() as i32;
    value+1
}

fn split_in_groups(rs_content: &Vec<String>) -> Vec<Vec<String>> {
    // Recorre el vector y lo separa en grupos de 3
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut temp: Vec<String> = Vec::new();
    let mut cont = 1;
    for line in rs_content {
        temp.push(line.to_string());
        if cont % 3 == 0 {
            groups.push(temp.clone());
            temp.clear();
        }
        cont += 1
    }
    groups
}

fn group_common_element(rs_group: &Vec<String>) -> char {
    // Get a group and get repeated elements between groups 
    let mut rs_map: HashMap<char, u32> = HashMap::new();
    for rs in rs_group {
        let mut seen: HashSet<char> = HashSet::new();
        for item in rs.chars().collect::<Vec<char>>() {
            if seen.contains(&item) {
                continue;
            }
            let i = rs_map.entry(item).or_insert(0);
            *i+=1;
            seen.insert(item);
        }
    }
    
    // Check for repeated elements between all 3 rs
    let mut shared_items: Vec<char> = Vec::new();
    for (k, v) in &rs_map {
        if *v == 3 {
            shared_items.push(*k);
        }
    }
    
    // TODO: Could check that there is exactly one or return an error?
    shared_items[0]
}

fn main() {
    let filename = "input_data.txt".to_string();
    let file_content: Vec<String> = read_file(filename);
    
    let mut total_prio = 0;
    for line in &file_content {
        total_prio += process_rucksack(line);
    };
    println!("Priority total of items: {total_prio}");

    let groups = split_in_groups(&file_content);
    let mut total_badge_value: i32 = 0;
    for g in &groups {
        let shared_items = group_common_element(&g);
        total_badge_value += item_value(&shared_items);
        println!("Shared item is {shared_items}");
    }
    println!("Total shared value is {total_badge_value}");
}

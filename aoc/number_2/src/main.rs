use std::fs::File;
use std::io::{BufRead, BufReader};

// A for Rock
// B for Paper
// C for Scissors

// X for Rock 1point        --> need to loose
// Y for Paper 2point       --> need to be a Draw
// Z for Scissors 3points   --> need to win
//
// 0 if tound lost
// 3 is draw
// 6 if win

fn read_file(filename: String) -> Vec<(String, String)> {
    // Read data to vector of raw plays
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut plays: Vec<(String, String)> = Vec::new();
    for line in reader.lines() {
        let parts: Vec<String> = line.unwrap().trim().split_whitespace().map(str::to_string).collect();
        match parts.len() {
            2 => {
                plays.push((parts[0].clone(), parts[1].clone()));
            },
            _ => println!("Otro largo no considerado"),
        }
    }
    plays
}

fn decide_round_1(choices: &(String, String)) -> u32 {
    // Compute the points in the round
    // This is the most lazy implementation of this (Just not to learn hashmaps!)
    let (opo, me) = choices;
    let points = match (opo.as_str(), me.as_str()) {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3 + 0,
        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 1 + 6,
        ("C", "Y") => 2 + 0,
        ("C", "Z") => 3 + 3,
        (_, _) => {
            println!("Algo raro con el valor");
            0
        }
    };
    points
}

fn decide_round_2(choices: &(String, String)) -> u32 {
    // Compute the points accoirding to elf instructions
    let (opo, me) = choices;
    match (opo.as_str(), me.as_str()) {
        ("A", "X") => 3 + 0, 
        ("A", "Y") => 1 + 3,
        ("A", "Z") => 2 + 6,
        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 2 + 0,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 1 + 6,
        (_, _) => {
            println!("Algo raro con el valor");
            0
        }
    }
}

fn main() {
    // Input data
    let raw_plays = read_file("input_data.txt".to_string());
    println!("{:}", raw_plays.len()); 
    
    let mut sum: u32 = 0;
    for (i, p) in raw_plays.iter().enumerate(){
        let points = decide_round_1(&p);
        sum += points;
    }
    println!("Total points gathered: {sum}");

    let mut sum_2: u32 = 0;
    for (i, p) in raw_plays.iter().enumerate(){
        let points = decide_round_2(&p);
        sum_2 += points;
    }
    println!("Total points gathered 2: {sum_2}");
}


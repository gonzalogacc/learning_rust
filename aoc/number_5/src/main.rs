use std::fs::File;
use std::io::{BufRead, BufReader};


fn load_data(filename: String) -> Vec<String> {
    // Load file 
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        data.push(line.unwrap());
    }
    data
}

#[derive(Debug)]
struct Move {
    cant: i32,
    from: i32,
    to: i32,
}

impl Move {
    fn new(cant: i32, from: i32, to: i32) -> Self {
        Self{
            cant, 
            from, 
            to
        }
    }
}

fn parse_towers(stack_data: &Vec<String>) -> Vec<Vec<String>> {
    // Read backwards to get the number of columns at the begining so i can build the structure
    let mut stack: Vec<Vec<String>> = Vec::new();
    for (i, line) in stack_data.iter().rev().enumerate() {
        if i == 0 {
            let columns: Vec<i32> = line
                .trim()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<_>>();
            // Add #columns empty vectors in the stack to put all 
            for _ in columns {
                stack.push(Vec::new());
            }
        } else {
            for ci in 0..stack.len() {
                let offset = 4;
                let index: usize = (ci*offset)+1;
                let package = line.chars().nth(index).unwrap();
                if package.is_whitespace() {
                    continue;
                }
                stack[ci].push(package.to_string());
            }
        }
    }
    stack
}

fn extract_stacking(data: &Vec<String>) -> (Vec<Vec<String>>, Vec<Move>) {

    let mut stack: Vec<String> = Vec::new();
    let mut cont: usize = 0;
    for line in data {
        cont += 1;
        if line.is_empty() {
            break;
        }
        stack.push(line.to_string());
    }
    let stack_data = parse_towers(&stack);

    let mut move_data: Vec<Move> = Vec::new();
    for line in &data[cont..] {
        let sline: Vec<_> = line.split_whitespace().collect();
        let cant: i32 = sline[1].parse().expect("cant not a number");
        let from: i32 = sline[3].parse().expect("from not a number");
        let to: i32 = sline[5].parse().expect("to not a number");
        move_data.push(Move::new(cant, from, to));
    }
    (stack_data, move_data)
}

fn apply_movement_9000(stack: &mut Vec<Vec<String>>, moves: &Vec<Move>) {
    // Single crate crane 
    for m in moves {
        for _ in 0..m.cant{
            let from: usize = (m.from as usize)-1;
            let to: usize = (m.to as usize)-1;

            let cand = stack[from].pop().unwrap();
            stack[to].push(cand);
        }
    }
    
    let mut result: String = String::new();
    for c in stack {
        result += c.last().unwrap();
    }
    println!("{}", result);
}

fn apply_movement_9001(stack: &mut Vec<Vec<String>>, moves: &Vec<Move>) {
    // Mutiple crates crane    
    for m in moves {
        //copy the last n
        let to: usize = (m.to as usize)-1;
        let from: usize = (m.from as usize)-1;
        let mut temp: Vec<String> = Vec::new();
        for _ in 0..m.cant{
            let cand = stack[from].pop().unwrap();
            temp.insert(0, cand);
        }
        stack[to].extend(temp);
    }
    
    let mut result: String = String::new();
    for c in stack {
        result += c.last().unwrap();
    }
    println!("{}", result);
}

fn main() {
    let data = load_data("input_data.txt".to_string()); 
    let (mut stack, moves) = extract_stacking(&data);

    //apply_movement_9000(&mut stack, &moves);
    apply_movement_9001(&mut stack, &moves);
    
}


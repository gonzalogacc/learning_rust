use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let mut max_cal = 0;
    let mut cal_sum: Vec::<u32> = vec![];
    match File::open("./real_data.txt") {
        Ok(infile) => {
            let mut calories: u32 = 0;
            let reader = BufReader::new(infile);
            for line in reader.lines() {
                let line_content = line.unwrap();
                // if line is empty that guy is done with counting, compare and reset
                if line_content.len() == 0 {
                    if calories > max_cal {
                        max_cal = calories;
                    }
                    cal_sum.push(calories);
                    calories = 0;
                } else {
                    let cals: u32 = line_content.trim().parse().expect("Not a number");
                    calories += cals
                }
            };
        },
        Err(_) => println!("Error file"),
    }
    println!("The elve with most calories i carrying {max_cal}");
    
    // Sort in reverse order
    cal_sum.sort();
    cal_sum.reverse();

    let mut first_three = 0;
    for (i, cal) in cal_sum.iter().enumerate() {
        if i>2 {
            break;
        }
        first_three += cal;
    }
    
    println!("The first 3 elves have {first_three}");
}

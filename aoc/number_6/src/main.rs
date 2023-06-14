use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_start_ok() {
        assert_eq!(find_start(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(find_start(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 60);
        assert_eq!(find_start(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(find_start(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }
}

fn load_buffer(filename: String) -> String {
    let buffer: String = String::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("{:}", line.unwrap());
    }
    String::new()
}

fn find_start(signal: &String) -> i32 {
    let point: i32 = 42;
    point

}

fn main() {
    load_buffer("test_data.txt".to_string());
}

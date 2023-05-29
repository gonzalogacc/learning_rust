fn print_labeled_measurements (x: i32, unit_label: char, name: String) {
    println!("The value of x is: {x}{unit_label}");
    println!("Vamos {name} carajo!");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}

fn main() {
    println!("Hello world!");
    //let name = "gonzalo".to_string();
    //print_labeled_measurements(5, 'h', name);   
    let x = five();
    println!("The value of five is: {x}");
    let y = plus_one(2);
    println!("The value of plusone is: {y}");
}

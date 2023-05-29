use std::io;

fn main() {
    let a: [i32; 5] = [1 ,2,3,4,5];

    println!("Please select an arrey index");

    let mut index: String = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Must be a number");

    let element = a[index];
    println!("The value of the element {index} is {element}");

}


fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2);
    let s = format!("{} {}!", m1, m2);
    println!("Ya calla ese maldito warning {s}");

    let mut vec: Vec<i64> = vec![1,2,3];
    vec.push(4);
    println!("Vec is {}", vec[3]);
}

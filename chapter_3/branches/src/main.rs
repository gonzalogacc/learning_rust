fn main() {
    let number = 6;
    if number % 4 ==0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }
    
    let condition = true;
    let number = if condition {5+2} else {6+2};
    println!("The alue f the number is {number}");
    
    let mut count = 0;
    let result = loop {
        println!("Again!");
        count+=1;
        if count > 10 {
            break count * 2;
        }
    };
    println!("The returned value is {result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LINFOTFF");

    let a = [10, 20, 30, 40, 50];
    println!("THe array a is of length {}", a.len());

    let mut index = 0;
    while index < a.len() {
        println!("The value at index {index} is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value of the element inside the for is {element}");
    }

    for numerin in (1..100).rev() {
        println!("{numerin}");
    }
}

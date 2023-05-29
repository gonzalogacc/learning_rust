
// Const usage type must be specific
const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

fn main() {
    
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x is: {x}");
    
    println!("Const is {THREE_HOURS_IN_SECONDS}");
    
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Number is {guess}");

    let x = 2.0;

    let y: f32 = 3.0;
    println!("Valores son {x}, {y}");

    let sum = 5+10;
    println!("Sum: {sum}");
    
    let t: bool = true;
    println!("Bool value is: {t}");
    
    let tup: (i32, f64, f32) = (1, 2.0, 3.0);
    let (xx, xy, xz) = tup;
    println!("The value of y is {xy}");
    let t0 = tup.0;
    println!("The value of x is {t0}");

    let arreglo: [i32; 5] = [1,2,3,4,5];
    let valor = arreglo[0];
    println!("Valor es {valor}");

}

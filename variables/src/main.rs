fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const TIME: i32 = 10 * 60; // only takes constant expression
    let y = 10;
    println!("The value of y is: {y}");
    let y = 10 * 2; // shadowing
    println!("The value of y is: {y}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
}

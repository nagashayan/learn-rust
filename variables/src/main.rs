fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const TIME = 10*60; // only takes constant expression
    let y = 10;
    let y = 10 * 2; // shadowing
}

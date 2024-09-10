fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const TIME: i32 = 10 * 60; // only takes constant expression
    let y = 10;
    println!("The value of y is: {y}");
    let y = "Shadowed"; // shadowing
    println!("The value of y is: {y}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    let _x = 2.0; // default f64, prefixing with _ eliminates the unused variable warning
    let _truncated = -5 / 3; // Result -1
    let _t: bool = true;
    let _c: char = 'Z';

    // Tuples
    let tup: (u32, bool, char) = (500, false, 'N');
    println!("The value of char is: {0}", { tup.2 });

    // Arrays
    let _a = [1, 2, 3];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // [3,3,3,3,3]
    print("The value of c.2 {c[2]}");
}

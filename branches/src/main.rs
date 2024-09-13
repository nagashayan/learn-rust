fn main() {
    println!("Hello, world!");
    let x = 6;
    let number = if x > 5 { 5 } else { 0 }; //else { "six" }; will throw error - both arms must be same type
    // if number { - will throw error, rust will not consider it as bool
    if number < 3 {
        println!("condition was true");
    } else if number < 0 {
        println!("It's a negative?")
    } else {
        println!("condition was false");
    }
}

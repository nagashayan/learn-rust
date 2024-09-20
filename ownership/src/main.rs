fn main() {
    println!("Hello, world!");
    let x: i32 = 10;
    let y = x;
    println!("x = {x} and y = {y}");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {s2}");
    //println!("s1 = {s1}"); //ownership error
    // Use references to solve that
    let s3 = &s2;
    println!("s2 = {s2}, s3= {s3}");
}

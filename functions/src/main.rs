fn main() {
    println!("Hello, world!");

    getY();
}

fn another_function(x: i32, c: char) -> i32 {
    println!("The value of x is: {x} and c: {c}");
    let y = {
        let a = 10;
        a + 5
    };
    println!("The value of y is {y}");
    y
}

fn getY() {
    let y = another_function(3, 'a');
    println!("The value of y in getY is {y}");
}

fn main() {
    let mut counter = 0;
    let result = loop {
        println!("Hello, world!");
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
    };
    println!("The result: {result} and counter: {counter}");
}

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

    counter = 0;
    'outer_loop: loop {
        println!("The counter: {counter}");
        if counter == 5 {
            break;
        }
        loop {
            if counter == 3 {
                break 'outer_loop;
            } else if counter > 4 {
                break;
            }
            counter += 1;
        }
    }
    println!("The counter: {counter}");

    let arr = [1, 2, 3, 4, 5];
    for a in arr {
        println!("{a}");
    }

    let mut i = 0;
    while i < 5 {
        println!("{i}");
        i += 1;
    }
}

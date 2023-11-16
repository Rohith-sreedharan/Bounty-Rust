fn main() {
    let x = 42;
    if x > 0 {
        println!("Positive");
    } else {
        println!("Non-positive");
    }

    for i in 0..5 {
        println!("Iteration {}", i);
    }

    let mut counter = 0;
    while counter < 3 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    let result = add_numbers(10, 20);
    println!("Result: {}", result);

    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    let message = "Hello, world!";
    println!("{}", message);

    // This is a comment
    /* Multi-line
    comment */

    let mut i = 0;
    loop {
        println!("Infinite Loop: {}", i);
        if i >= 5 {
            break;
        }
        i += 1;
    }

    let condition = true;
    if condition {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

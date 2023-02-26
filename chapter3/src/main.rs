fn mut_example() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    /*
    The value of x is: 5
    The value of x is: 6
     */
}

fn shadowing_example() {
    let x = 5;

    let x = x + 1;

    // Inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    /*
    The value of x in the inner scope is: 12
    The value of x is: 6
    */
}

#[allow(unused)]
fn float_example() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

#[allow(unused)]
fn math_example() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

#[allow(unused)]
fn bool_example() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

#[allow(unused)]
fn char_example() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

#[allow(unused)]
fn tuple_example() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    // Index tuples like this: tup.0
}

#[allow(unused)]
fn array_example() {
    let a = [1, 2, 3, 4, 5];
    fn main() {
        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];
    }
}

#[allow(unused)]
fn expression_example() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    // The value of y is: 4
}

fn if_let_example() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    // The value of number is: 5
}

fn breaking_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    // The result is 20
}

fn label_loops_example() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_range_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


fn main() {
    mut_example();
    println!("=============");
    shadowing_example();
    float_example();
    math_example();
    bool_example();
    char_example();
    tuple_example();
    array_example();
    expression_example();
    if_let_example();
    breaking_loop();
    label_loops_example();
    while_example();
    for_range_example();
}
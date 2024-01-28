fn main() {
    // Execution Flow contol
    // if/else
    let x = 5;
    if x == 5 {
        println!("x is 5");
    } else {
        println!("x is not 5");
    }

    // if/else if/else
    let y = 6;
    if y == 5 {
        println!("y is 5");
    } else if y == 6 {
        println!("y is 6");
    } else {
        println!("y is not 5 or 6");
    }

    // if as expression
    let z = if x == 5 {
        10
    } else {
        15
    };

    println!("z = {}", z);

    // match vs javascript switch
    let a = 5;
    match a {
        1 => println!("a is 1"),
        2 => println!("a is 2"),
        3 => println!("a is 3"),
        4 => println!("a is 4"),
        5 => println!("a is 5"),
        _ => println!("a is not 1, 2, 3, 4 or 5"),
    }

    // pattern matching with strings
    let b = "hello";
    match b {
        "hello" => println!("b is hello"),
        "goodbye" => println!("b is goodbye"),
        _ => println!("b is neither hello nor goodbye"),
    }

    // match as expression
    let c = match a {
        1 => "a is 1",
        2 => "a is 2",
        3 => "a is 3",
        4 => "a is 4",
        5 => "a is 5",
        _ => "a is not 1, 2, 3, 4 or 5",
    };
    println!("c = {}", c);
}   

fn main() {
    // Functions
    // Functions are fixed in size
    // Functions are allocated on the stack
    println!("Function example");
    // Function call
    foo();
    // Function call with parameters
    let x = 5;
    let y = 6;
    let z = multyply(x, y);
    println!("x*y={z}");
}   

// function declaration
fn foo() {
    println!("foo");
}

// function with parameters
fn multyply(x: i32, y: i32) -> i32 {
    return x * y;
}

// snake_case function names
fn divide_by(x: i32, y: i32) -> i32 {
    return x / y;
}

// implicit return
fn add(x: i32, y: i32) -> i32 {
    x + y
}
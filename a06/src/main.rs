fn main() {
    // Execution Flow Control 
    // loop
    let mut x = 0;
    loop { // infinite loop
        println!("x = {}", x);
        x += 1;
        if x == 10 {
            break;
        }
    }

    // while
    let mut y = 0;
    while y < 10 {
        println!("y = {}", y);
        y += 1;
    }

    // for avoiding control variables
    for z in 0..10 { // this goes from 0 to 9
        println!("z = {}", z);
    }

    // for iterating over arrays
    let arr = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }
}   

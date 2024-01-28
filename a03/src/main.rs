fn main() {
    //Data Types in Rust
    //Complex Types
    
    // Arrays
    // Arrays are fixed in size
    // Arrays are allocated on the stack
    println!("Array example");
    // Infered type
    let arr1 = [1, 2, 3]; 
    // Explicit type & initialization
    let arr2: [char; 3]; // [type; size]
    arr2 = ['a', 'b', 'c'];
    
    for i in 0..arr1.len() {
        println!("arr1[{}] = {}", i, arr1[i]);
    }

    for i in 0..arr2.len() {
        println!("arr2[{}] = {}", i, arr2[i]);
    }

    
    //Tuples
    //Tuples are fixed in size
    //Tuples are allocated on the stack
    println!("Tuple example");
    // Infered type
    let tup1 = (1, 2, 3);
    // Explicit type & initialization
    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    println!("tup1 = {:?}", tup1);
    println!("tup2 = {:?}", tup2);

    println!("Decostructing a tuple and an array");
    //Deconstruting a tuple
    let (x, y, z) = tup1;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {z}");

    //Deconstructing an array
    let [a, b, c] = arr2;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {c}");

    // Using _ to ignore values
    let (x, y, _) = tup2;

    // Using .. to ignore values
    let [a, ..] = arr1;
    println!("a = {}", a);

    //Structs
    //Structs are fixed in size
    //Structs are allocated on the stack
    println!("Struct example");
    // Infered type
    let user1 = User {
        username: String::from("enrique"),
        email: String::from("en@ort.com"),
    };
    

}   

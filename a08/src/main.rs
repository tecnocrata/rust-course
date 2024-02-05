fn main() {
    // String types
    // String slice
    // It is a reference to a part of a string
    // It is inmutable
    let s = "Hello, World!";

    // concatenation
    let s1 = "Hello, ";
    let s2 = "World!";
    let s3 = s1.to_string() + s2;
    println!("{}", s3);

    // String
    // It is a growable, mutable, owned, UTF-8 encoded string type
    // String is a namespace and from is a function
    let mut s = String::from("Hello, ");
    s.push_str("World!");
    println!("{}", s);

    // &str vs String
    // &str is a reference to a string
    // &str doesn't have push_str method
    // &str is a reference to a part of a string
    let x = "Hello, ";
    // x.push_str("World!"); // error: cannot borrow immutable local variable `x` as mutable
    // convert first to String
    let mut y = x.to_string();
    y.push_str("World!");

    // String can't be initialized with a number
    // let n = String::from(3);

    // mutable str& can be declared but it can't be mutated
    // confused? let's see the example
    // let mut z = "This string type can't be mutated";

}   


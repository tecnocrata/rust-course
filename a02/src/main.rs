fn main() {
    //Data Types in Rust
    //Primitive Types

    // These are scalar types
    // Scalar types represent a single value
    // Integer, Floating-point numbers, Booleans, Characters


    // Boolean is a scalar type
    let is_active: bool = true; // Occupies 1 byte in memory
    println!("is_active: {}", is_active);

    if is_active {
        println!("is_active is true");
    } else {
        println!("is_active is false");
    }

    // Char is a scalar type
    let a1: char = 'a'; // Occupies 4 bytes in memory
    println!("a1: {a1}");

    // Integer is a scalar type
    let i1: i8 = 127; // Occupies 1 byte in memory
    let i2: i16 = 32767; // Occupies 2 bytes in memory
    let i3: i32 = 2147483647; // Occupies 4 bytes in memory
    let i4: i64 = 9223372036854775807; // Occupies 8 bytes in memory
    let i5: i128 = 170141183460469231731687303715884105727; // Occupies 16 bytes in memory
    //isize and usize depend on the architecture of the computer
    let i6: isize = 9223372036854775807; // Occupies 8 bytes in memory
    let i7: usize = 18446744073709551615; // Occupies 16 bytes in memory


    //Float is a scalar type
    let f1: f32 = 3.14; // Occupies 4 bytes in memory
    let f2: f64 = 3.14; // Occupies 8 bytes in memory

    println!("i7: {i7}");
}

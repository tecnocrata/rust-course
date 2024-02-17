fn main() {
    // Structs principles
    // There is no class keyword in Rust. Instead, structs are used to create custom data types.
    // A struct is a way to create a custom data type that is composed of other data types.
    struct Animal {
        name: String,
        age: u8,
        is_hungry: bool,
        specie: String,
    }
    let dog = Animal {
        name: String::from("Buddy"),
        age: 5,
        is_hungry: true,
        specie: String::from("Dog"),
    };
    println!("{} is a {} and is {} years old", dog.name, dog.specie, dog.age);

    // it is not possible to display the value of a struct using println! macro
    // println!("dog: {}", dog); // error[E0277]: `Animal` doesn't implement `std::fmt::Display`
    // Option 1: In order to display the value of a struct, we need to implement the Display trait for the struct
    // like this example:
    impl std::fmt::Display for Animal {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{} is a {} and is {} years old", self.name, self.specie, self.age)
        }
    }

    println!("dog: {}", dog);

    // Option 2: use the Debug trait, which is a trait that is used for debugging purposes
    // The Debug trait is similar to the Display trait, but it is used for debugging purposes.
    // The next example shows how to implement the Debug trait for the Mamal struct:
    #[derive(Debug)]
    struct Mamal {
        name: String,
        age: u8,
        is_hungry: bool,
        specie: String,
    }
    let cat = Mamal {
        name: String::from("Kitty"),
        age: 3,
        is_hungry: true,
        specie: String::from("Cat"),
    };
    println!("cat: {:?}", cat);

}   


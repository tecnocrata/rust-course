fn main() {
    // Enums
    #[derive(Debug)] // this is needed to print the enum
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let player_direction: Direction = Direction::Down;
    println!("{:?}", player_direction);

    // Variants with data
    // Every element in an enum is called a variant
    // Variants can have data associated with them
    // This data can be of different types
    // This data is called fields
    #[derive(Debug)]
    enum PlayerAction {
        Move { direction: Direction, speed: u8 },
        Wait,
        Attack(String),
    }

    let player_action: PlayerAction = PlayerAction::Move {
        direction: Direction::Up,
        speed: 2,
    };
    println!("{:?}", player_action);

    // Enums with methods
    impl PlayerAction {
        fn print(&self) {
            match self {
                PlayerAction::Move { direction, speed } => {
                    println!("Player wants to move in direction {:?} with speed {}", direction, speed);
                },
                PlayerAction::Wait => {
                    println!("Player wants to wait");
                },
                PlayerAction::Attack(s) => {
                    println!("Player wants to attack {}", s);
                },
            }
        }
    };

    // This doesn't exists in any language I know
    // You can add methods to practically any type
    // This is called an impl block
    player_action.print();

    // Dynamic variants
    // Enums can have different types of data in their variants
    #[derive(Debug)]
    enum Shape {
        Rectangle { width: u32, height: u32 },
        Square(u32),
        Circle(f64),
    }

    // Variants with data can be dynamic
    // the variant is determined at runtime
    #[derive(Debug)]
    enum Color {
        RGB(u8, u8, u8),
        Red, // pure color
        Green, // pure color
        Blue    // pure color
    }
    // example of dynamic variant usage
    let red = Color::RGB(255, 0, 0);
    let green = Color::Green;
    let blue = Color::Blue;
    println!("{:#?}", red);
}   


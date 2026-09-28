#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 6.1: Defining an Enum
///
/// Enums let the programmer describe one value that can be one of several
/// related possibilities. This file continues the small game-world theme from
/// the structs and methods examples.
fn main() {
    choosing_between_variants();
    storing_data_in_variants();
    methods_for_enums();
}

/// A direction that a character can face or move toward.
enum Direction {
    North,
    South,
    East,
    West,
}

/// An action a player can take. Every value is exactly one of these variants.
enum PlayerAction {
    Move(Direction),
    Attack { target: String, damage: u32 },
    Say(String),
    Quit,
}

/// A message sent from the game client to its server.
enum ServerMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// # Choosing Between Variants
/// - An enum definition lists every possible variant of a new type.
/// - A value of an enum can be only one variant at a time.
/// - Variants are named with `EnumName::VariantName`.
/// - Unlike the fields of a struct, each enum variant can carry its own
///   values: no data, one unnamed value, several unnamed values, or named
///   fields.
fn choosing_between_variants() {
    println!("\n{:=>80}", "");
    println!("choosing_between_variants()\n");

    let hero_direction = Direction::North;
    let monster_direction = Direction::West;

    describe_direction(hero_direction, "The hero");
    describe_direction(monster_direction, "The monster");

    // A Direction value must be one of the variants declared above.
    // let invalid_direction = Direction::Up; // Uncomment to see the error.
}

/// # Storing Data in Variants
/// - Variants can hold data, so one enum can replace several related structs.
/// - A variant can contain no data, named fields, unnamed fields, or one value.
/// - Each variant chooses the data that makes sense for that possibility.
/// - The enum name groups these different action values under one type.
fn storing_data_in_variants() {
    println!("\n{:=>80}", "");
    println!("storing_data_in_variants()\n");

    let walk = PlayerAction::Move(Direction::East);
    let warning = PlayerAction::Say(String::from("Beware, there is a wild spooderman!"));
    let strike = PlayerAction::Attack {
        target: String::from("Spooderman"),
        damage: 12,
    };
    let leave_game = PlayerAction::Quit;

    print_action(walk);
    print_action(warning);
    print_action(strike);
    print_action(leave_game);

    // Each variant accepts only the data in its definition.
    // let invalid_action = PlayerAction::Move(10); // Uncomment to see the error.
}

/// # Methods for Enums
/// - Like structs, enums can have `impl` blocks and associated methods.
/// - A method can inspect `self` to determine which variant it received.
/// - `match` handles the different possibilities. It will be explored in the
///   next chapter.
fn methods_for_enums() {
    println!("\n{:=>80}", "");
    println!("methods_for_enums()\n");

    let move_message = ServerMessage::Move { x: 4, y: -2 };
    let chat_message = ServerMessage::Write(String::from("A party member joined."));
    let color_message = ServerMessage::ChangeColor(80, 160, 255);
    let quit_message = ServerMessage::Quit;

    move_message.call();
    chat_message.call();
    color_message.call();
    quit_message.call();
}

// NOTE: The helper functions below use `match` to inspect enum variants.
// We will cover match expressions in more detail in the upcoming materials.

/// Prints a readable description of a direction.
fn describe_direction(direction: Direction, character: &str) {
    let direction_name = match direction {
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
    };

    println!("{character} is facing {direction_name}.");
}

/// Prints a readable description of a player action.
///
/// Taking `PlayerAction` by value moves it into this function, which is
/// appropriate because this example only prints each action once. A function
/// that needs the caller to keep using the action could borrow `&PlayerAction`
/// instead. This applies to the `describe_direction` function as well.
fn print_action(action: PlayerAction) {
    match action {
        PlayerAction::Move(direction) => describe_direction(direction, "The player"),
        PlayerAction::Attack { target, damage } => {
            println!("The player attacks {target} for {damage} damage.");
        }
        PlayerAction::Say(message) => println!("The player says: \"{message}\""),
        PlayerAction::Quit => println!("The player leaves the game."),
    }
}

impl ServerMessage {
    /// Handles the message according to the data in its variant.
    fn call(self) {
        match self {
            Self::Quit => println!("The client disconnected."),
            Self::Move { x, y } => println!("Move the character to ({x}, {y})."),
            Self::Write(text) => println!("Show chat text: \"{text}\""),
            Self::ChangeColor(red, green, blue) => {
                println!("Set the interface color to rgb({red}, {green}, {blue}).");
            }
        }
    }
}

#[derive(Debug)]
enum WebEvent {
    PageLoad, // unit variant i.e. no data associated
    PageUnload,
    KeyPress(char),
    Paste(String),            // tuple variant
    Click { x: i64, y: i64 }, // struct variant
}

/*

enums are types which have a few definite values and can hold data
tagged unions (discriminant + data)
discriminant = value which tells which variant is used

size = size of largest variant + size of discriminant
but due to alignment requirement, rust may add padding (depends on the platform architecture)
*/

// defining methods on enums
impl WebEvent {
    fn log(&self) {
        // match is used to compare the value of self with each variant
        match self {
            // type aliasing e.g. type test = WebEvent;
            Self::PageLoad => println!("Page loaded"),
            Self::PageUnload => println!("Page unloaded"),
            Self::KeyPress(c) => println!("Key pressed: {}", c),
            Self::Paste(s) => println!("Pasted: {}", s),
            Self::Click { x, y } => println!("Clicked at ({}, {})", x, y),
        }
    }
}

// built in enums

// Option<T> is either Some value of type T or None
fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

// Result<T, E> is either Ok value of type T or Err value of type E
fn divide_result(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(x / y)
    }
}

// implicit discriminator starting at 0
enum Numbers {
    One, 
    Two,
    Three,
}

 // explicit discriminator
enum Color {
    Red = 3,
    Green, // 4
    Blue = 2,
    // Yellow,  will lead to error
}

fn main() {
    let event = WebEvent::Paste(String::from("Hello, world!"));
    event.log();

    // type aliasing
    type test = WebEvent;
    // we'll be able to use test::PageLoad, test::PageUnload, etc.
    
    // Click already brought into scope using "use"
    let c = Click { x: 20, y: 80 };
    
    // use declaration is used to bring the enum into scope
    use crate::WebEvent::{Click, PageLoad, Paste};
    let event2 = PageLoad;
    event2.log();

    // println!("{}", PageLoad as i32); // won't work as enum variants are not integers
    println!("{}", Numbers::One as i32); // works as enum variants are integers
    println!("{}", Color::Green as i32); // 4
}

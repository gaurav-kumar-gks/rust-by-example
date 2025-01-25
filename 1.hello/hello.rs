fn main() {
    // compile -> "rustc hello.rs"
    // run -> "./hello"
    // println! is macro
    println!("Hello, world!");

    // various print
    // format! (return formatted string)
    // print! and println! (print! with new line)
    // eprint! and eprintln!

    println!(
        "My name is {name:*>8}:{code:$>5}\nToday is {0}\n
        I have b{1:b} cookies with me yay\n",
        "Wednesday",
        10,
        name = "Alexa",
        code = 1010
    );
}

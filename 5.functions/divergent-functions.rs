/*
Divergent functions

- Functions that do not return
- `!` is the return type
- `panic!` is a divergent function
- `loop` is a divergent function
- used typically in infinite loops or panics
*/


// divergent function
fn divergent() -> ! {
    panic!("This function never returns!");
}

// divergent function
fn divergent_loop() -> ! {
    loop {
        println!("This function never returns!");
    }
}

fn main() {
    // divergent(); // panics and exits
    // divergent_loop(); // infinite loop
}
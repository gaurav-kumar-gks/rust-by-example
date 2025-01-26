/*

Closures:

- annonymous functions
- can capture variables from the surrounding environment
- can be stored in variables
- single expression body - return type and braces can be omitted - type inference
- once type is inferred, it is fixed
- closure preferentially captures variables by referene unless required otherwise

|args| -> return_type {
    body
}

|args| body
*/


// Fn: immutable borrow
fn call_fn<F: Fn()>(f: F) {
    f();
}

fn another_function() {
    println!("Hello from another function");
}

// same can be written with 'where' 
// 'where' is used to specify bounds on generic types
// fn call_fn<F>(f: F) where F: Fn() { f(); } 

// FnMut: mutable borrow
fn call_fn_mut<F: FnMut()>(mut f: F) {
    f();
}

// FnOnce: takes ownership
fn call_fn_once<F: FnOnce()>(f: F) {
    f();
}

// returns a closure that captures x
fn create_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

// returns a closure that captures x
// need to use `impl` to return a closure
// need to use `move` to capture x
// otherwise, x will be dropped when the function returns
fn create_fnonce(x: i32) -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {} {}", text, x)
}

fn main() {
    let x = 4;
    let add_x = |y| x + y;
    println!("add_x(2): {}", add_x(2)); // Output: add_x(2): 6

    let add_3 = |a| -> i32 { a + 3 };
    println!("add_3(2): {}", add_3(2)); // Output: add_3(2): 5

    let print = || println!("Hello from closure");
    // print(); // Output: Hello from closure

    // Capture by reference
    let a = 10;
    let print_a = || println!("a: {}", a);
    print_a(); // Output: a: 10

    // Capture by mutable reference
    let mut b = 20;
    let mut add_to_b = |z| b += z;
    add_to_b(5);
    println!("b: {}", b); // Output: b: 25

    // Capture by value
    // `move` forces closure to take ownership of captured variables
    let c = 30;
    let consume_c = move || println!("c: {}", c);
    consume_c(); // Output: c: 30
    // println!("c: {}", c); // Error: `c` has been moved

    let add_n = create_adder(5); // returns a closure
    let result = add_n(10); // calling a closure
    // println!("Result: {}", result); // Output: 15

    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");
    call_fn(closure);
    call_fn(another_function); // another_function also satisfies the `Fn` bound

    let create_once = create_fnonce(10);
    create_once();
}

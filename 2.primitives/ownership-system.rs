/*

Ownership System

ownership is rust's most unique feature
it enables rust to make memory safety guarantees w/o needing a garbage collector

ownership:
- each value in rust has a variable that's called its owner
- each value has a single owner
- when the owner goes out of scope, the value will be dropped

borrowing:
- borrowing allows you to refer to a value without taking ownership
- immutable (default)(&T) and mutable (&mut T) borrowing
- multiple immutable references can be made to a value
- only one mutable reference can be made to a value
- can't have multable and immutable reference simultaneously

moving:
- passing a variable to a function will move or copy it
- when a value is moved, it can no longer be used
- moving a value is done by assigning it to another variable or passing it to a function
- to avoid moving, use references

slices:
- slices are references to a contiguous sequence of elements in a collection

copy:
- types that are stored on the stack are copied when assigned to another variable
- e.g. integers, floats, bools, char, tuples (if they contain types that are copy)

clone:
- types that are stored on the heap are moved when assigned  to another variable
- e.g. String, Vec<T>
- to make a copy of a value on the heap, use the clone method

*/

// s is moved
fn call_fn_with_move(s: String) {
    println!("{}", s);
}

// s is borrowed immutably
fn call_fn_with_borrow(s: &String) {
    // s.push_str(" (function modified this string)"); // Error: cannot modify a immutable borrowed value
    println!("{}", s);
}

// s is borrowed mutably
fn call_fn_with_mutable_borrow(s: &mut String) {
    s.push_str(" (function modified this string)");
}

fn main() {
    let s1 = String::from("this string is moved to another variable"); // s1 owns the String
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2); // Works: s2 owns the String

    let s3 = String::from("passing string to function by moving"); // s3 owns the String
    call_fn_with_move(s3); // s3 is moved to test, s3 is no longer valid
    // println!("{}", s3); // Error: s3 is no longer valid

    let s4 = String::from("passing string to function by immutable reference"); // s4 owns the String
    call_fn_with_borrow(&s4); // s4 is borrowed, s4 is still valid
    println!("{}", s4); // Works: s4 owns the String

    let mut s5 = String::from("passing string to function by mutable reference"); // s5 owns the String
    call_fn_with_mutable_borrow(&mut s5); // s5 is borrowed mutably, s5 is still valid
    println!("{}", s5); // Works: s5 owns the String
}
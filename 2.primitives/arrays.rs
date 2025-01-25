use std::mem;

fn main() {
    // array: fixed-size collection of same type elements
    // allocated on stack - which is faster than heap
    // Fixed-size array (type signature is superfluous)
    // [T; length] -> T:type length:compile-time constant
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // immutable by default
    // arr[0] = 10; // Error!
    // can create a mutable array using mut keyword

    // overwritten with shadowing.
    let arr = [10, 20, 30, 40, 50];

    // init with same values
    let arr = [10; 5];

    println!("Array elements: ");
    for element in arr.iter() {
        print!("{} ", element);
    }
    println!();


    // some methods of array
    println!("Array: {:?}", arr);
    println!("Second element of array: {}", arr[1]);
    println!("Length of array: {}", arr.len());
    println!("Size of array: {}", mem::size_of_val(&arr));
    // println!("first element of array: {}", arr.first()); // Error: .first() returns Option<&T>
    println!("last element of array: {}", arr.last().unwrap()); 
    
    // to use .first() or .last() we need to match the return value
    // or to use arr.get(i)
    match arr.first() {
        Some(value) => println!("First element: {}", value),
        None => println!("Array is empty"),
    }

    println!("Array contains 30? {}", arr.contains(&30));
    let _x: Option<&i32> = arr.iter().max();
    // println!("Max element: {}", x.unwrap());

    match arr.iter().position(|&x| x == 30) {
        Some(index) => println!("Position of 30: {}", index),
        None => println!("30 not found in array"),
    }

    // slice: dynamically-sized view into a contiguous sequence
    // slice: &[T] -> T:type
    // it's a reference, they don't own the data they point to
    let slice = &arr[1..3];
    println!("Slice: {:?}", slice);
}

fn main() {
    // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);

    // Accessing elements of a tuple -> my_tuple.0, my_tuple.1, ...
    // Destructuring a tuple
    let (a, b, c, d) = my_tuple;

    // Tuple can be used as function arguments
    let pair = (10, true);
    println!("pair: {:?}", pair);

    // long tuple > 12 elements can't be printed
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("first element of too_long_tuple: {}", too_long_tuple.0);
    // println!("Too long tuple: {:?}", too_long_tuple); // Error!
}
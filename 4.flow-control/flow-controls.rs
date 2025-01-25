fn main() {
    /* if else */
    let x = 10;
    if x > 15 {
        println!("x is greater than 15");
    } else if x > 5 {
        println!("x is greater than 5 but less than or equal to 15");
    } else {
        println!("x is less than or equal to 5");
    }

    // if as an expression (returns a value)
    let result = if x > 5 {
        // println!("inside if");
        "if block result"
    } else {
        "else block result"
    };

    /* match */
    let array = [1, 2, 3];

    match array {
        [0, s, t] => println!("0, {}, {}", s, t),
        [1, ..] => println!("1, .."),
        [first, .., last] => println!("{}, .., {}", first, last),
        [.., last] => println!(".., {}", last),
        [first, mid @ .., last] => println!("{}, {:?}, {}", first, mid, last),
        _ => println!("unknown"),
    }

    let t = 4;
    let result = match t {
        1 if x <= 1 => "one", // guard
        2 => "two",
        3 => "three",
        n @ 4..=5 => &format!("{:?} is b/w four to five", n), // binding
        6 | 7 => "six or seven",
        _ => "unknown", // default
    };

    /* loop - infinite loop */
    let mut count = 0;

    loop {
        println!("Count: {}", count);
        count += 1;

        if count == 5 {
            break; // Exit the loop
        }
    }

    // loop as an expression (returns a value)
    let result = loop {
        count += 1;
        if count & 1 == 1 {
            break count; // Return the value of count
        }
    };

    /* while */
    // while x < 5 {}

    /* for */
    let arr = [1, 2, 3, 4, 5];
    for i in 0..5 {}
    for i in 0..=5 {}
    for i in arr.iter() {}
    for i in arr.iter().rev() {}
    for i in arr.iter().step_by(2) {}
    for (index, value) in arr.iter().enumerate() {}

    /* let inside if / while */
    // if let Some(value) = arr.first() {}
    // while let Some(value) = arr.first() {}
    // let Some(x) = some_value else {}

    /* nested loops and labels */

    'outer: for i in 0..2 {
        'inner: for j in 0..=1 {
            if i == 1 && j == 1 {
                break 'outer; // Break the outer loop
            }
            println!("i: {}, j: {}", i, j);
        }
    }

    let v1 = vec![1, 2, 3];
    for i in v1.iter() {
        println!("{}", i);
    }
    println!("Original vector: {:?}", v1);

    // consuming the vector
    let v2 = vec![1, 2, 3];
    for i in v2.into_iter() {
        println!("{}", i);
        break;
    }
    // The original vector is no longer valid as it has been consumed
    // println!("Original vector: {:?}", v2); // Error!

    let mut v3 = vec![1, 2, 3];
    for i in v3.iter_mut() {
        *i *= *i;
    }
    println!("Original vector: {:?}", v3);
}

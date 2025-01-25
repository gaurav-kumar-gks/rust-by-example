// const
// don't skip the data type
// or assign a value
const THREE: i32 = 3; 

struct Unit;

#[derive(Debug)]
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: Pair,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

/*

all fields are stored in single block of memory
size of struct = sum of sizes of all fields
do not have a discriminant like enums

*/

fn main() {
    let unit = Unit;

    let pair: Pair = Pair(1, 0.1);
    println!("Pair: {:?}", pair);
    // destructuring
    let Pair(integer, decimal) = pair;

    let point: Point = Point {
        x: 0.0,
        y: 0.0,
        z: Pair(1, 2.0),
    };
    // destructuring
    let Point { x, y, z } = point;

    println!("Point: {:?}", point);
    println!("Point.z: {:?}", point.z);

    let top_edge = 0.0;
    let left_edge = 0.0;
    let rectangle: Rectangle = Rectangle {
        top_left: Point {
            x: top_edge,
            y: left_edge,
            z: Pair(1, 2.0),
        },
        bottom_right: Point {
            x: 5.0,
            y: -5.0,
            z: Pair(3, 4.0),
        },
    };
    println!("Rectangle: {:?}", rectangle);
}

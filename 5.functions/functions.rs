// Function
// no restriction on order of fn declarations
fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
// funcs that "don't" return a value
// actually return `()`

#[derive(Debug)]
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // associated function
    // no self parameter
    // don't need to be called on an instance
    // used generally like constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // associated function
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    // method
    // self: &Self -> immutable borrow
    fn area(&self) -> f32 {
        println!("in area");
        println!("top_left: {:?}", self.top_left);
        println!("bottom_right: {:?}", self.bottom_right);
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.top_left.y - self.bottom_right.y;
        return width * height
    }

    // associated function
    fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    // needs caller to be mutable -> otherwise error
    fn translate(&mut self, x: f32, y: f32) {
        self.top_left.x += x;
        self.bottom_right.x += x;
        self.top_left.y += y;
        self.bottom_right.y += y;
    }

    fn copy(&self) -> Rectangle {
        Rectangle {
            top_left: self.top_left.clone(),
            bottom_right: self.bottom_right.clone(),
        }
    }

    // self: Self -> takes ownership
    // basically destructor
    fn destroy(self) {
        println!("Rectangle destroyed");
    }

    // self: Self -> takes ownership
    // returns the value
    fn destroy_and_return(self) -> Rectangle {
        println!("Rectangle destroyed");
        self
    }
}

fn main() {
    let result = fib(3);

    let r = Rectangle::new(Point::origin(), Point::new(3.0, 4.0));
    // println!("Area of rectangle: {}", r.area());
    println!("Rectangle: {:?}", r);

    let r2 = r.destroy_and_return();
    println!("Rectangle: {:?}", r2);
    println!("Area of rectangle: {}", r2.area());

    // println!("Translating rectangle by (1, 1): ", r.translate(1.0, 1.0)); // will throw err
}  

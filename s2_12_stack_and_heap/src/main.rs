use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

fn main() {
    let x = Box::new(5);
    // Requires dereferencing the box to get the value.
    println!("x = {}", *x);

    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3.x = {}", p3.x);
}

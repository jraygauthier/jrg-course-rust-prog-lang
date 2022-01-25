#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    let a: u8 = 123;
    println!("a = {}", a);

    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",
             z, size_of_z, size_of_z*8);

    let d: char = 'x';
    println!("d = {} is a char, size = {} bytes", d, mem::size_of_val(&d));

    let e: f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let sov = mem::size_of_val;

    let g: bool = false;
    println!("g = {}, size = {} bytes", g, sov(&g));
}

use std::{mem, usize};

use std;

fn arrays()
{
    // Redundant type declaration.
    // let mut a:[i32; 5] = [1, 2, 3, 4, 5];
    let mut a = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}",
        a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    // Nice debug print notation to print the full array:
    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5]
    {
        println!("does not match");
    }

    if a != [321, 2, 3, 4, 5]
    {
        println!("match");
    }

    // Cannot compile with array of different lenght.
    // if a != [1, 2, 3, 4, 5, 6]

    let b = [1; 10];
    for i in 0..b.len()
    {
        println!("b[{}] = {}", i, b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
        }
    }
}

fn main() {
    arrays();
}

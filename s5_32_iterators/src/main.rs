fn main() {
    let mut vec = vec![3, 2, 1];

    for x in &vec
    {
        println!("*x = {}", *x);
    }

    for x in vec.iter()
    {
        println!("x = {}", x);
        println!("*x = {}", *x);
    }

    for x in vec.iter_mut()
    {
        *x += 2;
    }
    println!("{:?}", vec);

    for x in vec.iter().rev()
    {
        println!("in reverse: {}", x);
    }

    let mut vec2 = vec![1, 2, 3];
    // `extend` uses `vec.into_iter` and thus moves
    // all elements from `vec` to `vec2`.
    vec2.extend(vec);

    println!("{:?}", vec2);
    // Would fail with error "value borrowed here after move".
    // println!("{:?}", vec);
}

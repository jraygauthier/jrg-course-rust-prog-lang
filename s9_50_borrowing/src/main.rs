fn main() {
    // Taking the vector by reference. It allows this closure
    // to borrow it from its caller.
    let print_vector = |x:&Vec<i32>|
    {
        println!("x[0] = {}", x[0]);
    };

    let v = vec![3, 2, 1];
    // Here we're passing a reference to v to our closure. We're effectively
    // lending it the vector.
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    // According to presenter, borrowing a mutable as immutable
    // is bad even tough allowed by the compiler.
    // I am not sure I understand well the point.
    // let c = &a;
    {
        // `b` borrows `a`.
        let b = &mut a;
        // We cannot use `a` until `b` un-borrows it (i.e: gives it back).
        // println!("a = {}", a);
        *b += 2;
    }
    // which occurs here:
    println!("a = {}", a);

    let mut z = vec![3, 2, 1];

    for i in &z
    {
        println!("i = {}", i);
        // Rust nicely prevent this with error "cannot borrow `z` as mutable
        // because it is also borrowed as immutable". This is quite nice
        // as pushing to the vector during iteration on it would cause
        // undefined behavior.
        // z.push(5);
    }
}

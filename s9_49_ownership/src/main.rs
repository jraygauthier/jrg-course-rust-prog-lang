fn main() {
    // Vector `v` owns the memory that is stored in the vector. The vector is on
    // the stack, however the memory for storing its elements is on the stack.
    let v = vec![1, 2, 3];

    let v2 = v;

    // This is not allowed as `v` memory has been moved to `v2` (its new owner).
    // Error will be "value borrowed here after move".
    // println!("{:?}", v);

    let foo = |v:Vec<i32>| ();
    // This is not allowed. Error is "value used after move".
    // foo(v);

    let u = 1;
    let u2 = u;
    // Not however that it works fine with primitive values:
    println!("{:?}", u);
    // This is because i32 does not have any pointer to other data
    // (does not own anything outside of its stack space).

    let bu = Box::new(1);
    let bu2 = bu;
    // We see that with a boxed i32, we get the same problem:
    // println!("{:?}", *bu);

    let print_vector = |x:Vec<i32>| -> Vec<i32>
    {
        // We prints the vector and returns it back so that it can be reused.
        println!("{:?}", x);
        x
    };

    let z = vec![1, 2, 3];
    let zz = print_vector(z);
    // This works fine as `print_vector` returned ownership of the instance via
    // its return value.
    println!("{}", zz[0]);

    // However, this is quite inconvenient and this is why borrowing exists in
    // the first place.
}

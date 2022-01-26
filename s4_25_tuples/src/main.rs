
fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}

fn tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);

    // Accessing tuple elements via indices;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // Destructuting the tuple;
    let (a,b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    // Cannot access it directly as `combined.1.1`.
    // Parenthesises are required.
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // `(42)` is not a tuple. We should use same
    // trailing comma trick as python to get this:
    let meaning = (42,);
    println!("{:?}", meaning);
}

fn main() {
    tuples();
}

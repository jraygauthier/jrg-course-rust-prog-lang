trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("string: {}", *self)
    }
}

// This gets monomorphized: we get type specific
// implementation.
fn print_it<T: Printable>(z: T)
{
    println!("{}", z.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();

    println!("{}", a.format());
    println!("{}", b.format());

    // This uses static dispatch. Compiler calls the
    // type specific / monomorphized implementation.
    print_it(a);
    print_it(b);
}

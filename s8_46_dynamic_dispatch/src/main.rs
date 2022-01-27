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

fn print_it(z: &dyn Printable)
{
    // We no longer know the specific type here. Type has been erased. Dynamic
    // dispatch is performed here making this a more expansive call.
    println!("{}", z.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();

    // Our new implementation uses dynamic dispatch. Type erasure occurs before
    // the call.
    print_it(&a);
    print_it(&b);
}

struct Person
{
    name: String
}

impl Person
{
    // fn new(name: &str) -> Person
    // {
    //     Person { name: name.to_string() }
    // }

    // fn new<S: Into<String>>(name: S) -> Person
    fn new<S>(name: S) -> Person
        where S: Into<String>
    {
        Person { name: name.into() }
    }
}

fn main() {
    println!("Hello, world!");

    let john = Person::new("John");
    let name: String = "Jane".to_string();

    // No longuer needs the `as_ref` now that api
    // supports both.
    let jane = Person::new(name/*.as_ref()*/);
}

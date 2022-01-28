struct Person<'a>
{
    // Make `name` lifetime same as `self`.
    name: &'a str
}

// We also need to make sure that the implementation
// lifetime spec is the same as that of the `Person`.
impl<'a> Person<'a>
{
    fn talk(&self)
    {
        println!("Hi, my name is {}", self.name);
    }
}

fn main() {
    let person = Person{ name: "Dmitri" };
    person.talk();
}

use std::rc::Rc;

struct Person
{
    name: Rc<String>
}

impl Person
{
    fn new(name: Rc<String>) -> Person
    {
        Person { name: name }
    }

    fn greet(&self)
    {
        println!("Hi, my name is {}", self.name);
    }
}

fn rc_demo()
{
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        // `clone` needs to be used here to prevent move
        // semantic. This will lead to `Rc`'s ref count
        // being incremented.
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));

    // Before moving to `Rc<String>` we had this error:
    // Error: "value borrowed here after move".
    // But now, using `Rc`, we are able to acheive it.
    println!("Name = {}", name);
}

fn main() {
    rc_demo();
}

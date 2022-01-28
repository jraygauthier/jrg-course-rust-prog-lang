use std::sync::{Arc, Mutex};
use std::thread;

struct Person
{
    name: Arc<String>,
    // Here we use `Mutex` to protect our string.
    state: Arc<Mutex<String>>
}

impl Person
{
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person
    {
        Person { name: name, state: state }
    }

    fn greet(&self)
    {

        // Simple using a `Arc<String>` for the `state`, rustc failed with the
        // "cannot borrow data in an `Arc` as mutable" error.
        // self.state.clear();
        // self.state.push_str("excited");

        // However, with the `Mutex` wrapper, everything is now fine.
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");

        println!("Hi, my name is {}", self.name);
    }
}

fn arc_demo()
{
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, state = {}",
        name,
        state.lock().unwrap().as_str()
    );

    t.join().unwrap();
}

fn main() {
    arc_demo();
}

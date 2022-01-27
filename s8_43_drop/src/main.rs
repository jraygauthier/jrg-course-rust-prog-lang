struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    // Usually not very useful, mostly nice for debugging.
    fn drop(&mut self) {
        println!("{} is dead", self.name)
    }
}

fn main() {
    let goblin = Creature::new("Jeff");

    println!("game proceeds");

    // This is disallowed:
    // goblin.drop();
    // But this is:
    drop(goblin);

    // This won't be allowed after above explicit `drop`.
    // println!("goblin name is {}", goblin.name);

    let clever: Creature;
    {
        let clever_goblin = Creature::new("Clever Bob");
        println!("clever game proceeds");
        clever = clever_goblin;
        println!("end of clever scope");
    }

    println!("clever name is {}", clever.name);
}

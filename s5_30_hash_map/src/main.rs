use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides",
        shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    // Insert a value only if it does not already exists.
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes
            .entry("circle".into())
            .or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

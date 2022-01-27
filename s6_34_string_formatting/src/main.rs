use std::fmt::format;

fn main() {
    let name = "Dmitri";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    let info = format!(
        "the name's {last}. {first} {last}.",
        first="james",
        last = "bond"
    );
    println!("{}", info);

    // Note that the compiler checks that arguments
    // in the format string are effectively passed.
    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        // Adding this arg wouldn't compile.
        // "gamma", // unused
        data = "delta"

    );
    println!("{}", mixed);
}

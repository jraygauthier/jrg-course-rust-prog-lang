// It is possible to use our lib create from
// our bin crate.
use s12_61_comments_and_doc::greetings;

fn main() {
    let username = "John";
    println!("{}, {}!",
        greetings::hello(),
        username
    );
}

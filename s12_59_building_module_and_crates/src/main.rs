// No longer needed since rust 2018.
// extern crate phrases;
use phrases;

use phrases::greetings::french;

fn main() {
    println!("English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye()
    );

    println!("Français: {}, {}",
        french::hello(),
        french::goodbye()
    );
}

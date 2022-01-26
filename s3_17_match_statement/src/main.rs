
fn main() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("The country with code {} is {}",
        country_code, country);

    let x = false;

    let s = match x {
        true => "yes",
        // If I comment the below case, I would get a
        // "non-exhaustive pattern" compiler error.
        false => "no"
    };

    println!("s = {}", s);
}

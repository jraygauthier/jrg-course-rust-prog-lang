const MEANING_OF_LIFE: u8 = 42; // No fixed address.

static Z: i32 = 123;

static mut Z_MUT: i32 = 123;

fn main() {
    // `MEANING_OF_LIFE` is inlined.
    println!("MEANING_OF_LIFE = {}", MEANING_OF_LIFE);

    println!("Z = {}", Z);

    // As `Z_MUT` is mutable, rust complains by default as
    // accessing this variable is unsafe. We thus need to perform
    // this access from within an `unsafe` block / scope.
    unsafe {
        println!("Z_MUT = {}", Z_MUT);
        Z_MUT = 456;
        println!("Z_MUT = {}", Z_MUT);
    };
}

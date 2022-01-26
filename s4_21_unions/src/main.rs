// takes only 32 bits.
// (no tag is stored to indicate the case/type).
union IntOrFloat
{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            },
            IntOrFloat { f } => {
                println!("value = {}", f);
            }
        }
    }
}

fn main() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    // As rust unions are not *tagged* unions, accessing the data
    // requires an `unsafe` block.
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat{i: 42});
    process_value(IntOrFloat{f: 42.0});
    // This won't work well however:
    process_value(IntOrFloat{i: 5});

    // Use `union` only for C interrop. Otherwise, use `enum`
    // which are actually our tagged unions.
}


enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    Cmyk{cyan:u8, magenta:u8, yellow: u8, black: u8} // struct
}

fn enums()
{
    // let c:Color = Color::Red;
    // let c:Color = Color::RgbColor(10, 0, 0);
    let c:Color = Color::Cmyk{cyan: 0, magenta: 128, yellow: 0, black: 0};
    match c
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r, g, b),
        Color::Cmyk{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        _ => ()
    }
}

fn main()
{
    enums();
}

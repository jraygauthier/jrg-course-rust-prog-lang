fn how_many(x:i32) -> &'static str
{
    match x
    {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9..=11 => "lots of",
        _ if (x % 2 == 0) => "some even number of",
        _ => "a few"
    }
}

struct Color
{
    r: i32, g: i32, b: i32
}

pub fn pattern_matching()
{
    for x in 0..13
    {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    // let point = (3, 4);
    // let point = (5, 0);
    let mut point = (0, 7);

    match point
    {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        // pass `x` by reference:
        (ref x, 0) => println!("y axis, x = {}", x),
        // pass `y` by mutable reference. Only possible if
        // `point` is mutable.
        (x, ref mut y) => println!("({}, {})", x, y)
    }

    let color = Color{r:0, g:255, b:255};
    match color
    {
        // We can even focus only on a single struct attribute
        // diregarding the rest altogheter.
        Color{r: 0, ..} => println!("No red!"),
        _ => println!("Some color!")
    }
}

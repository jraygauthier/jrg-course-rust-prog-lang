// use std::vec;

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);

    println!("a = {:?}", a);

    let idx: usize = 0;

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    // Return an option type when out of bound index.
    match a.get(6)
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    if let Some(x) = a.get(6)
    {
        println!("a[6] = {}", x);
    }

    for x in &a { println!("{}", x); }

    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop();
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}

fn main() {
    vectors();
}

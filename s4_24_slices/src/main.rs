fn use_slice(slice: &mut[i32])
{
    println!("first elm = {}, len = {}", slice[0], slice.len());
    // As the slice is taken as mutable, it becomes
    // possible for us to change the original array's
    // through the slice.
    slice[0] = 4321;
}

fn slices()
{
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    // Possible to take the entire array:
    // use_slice(&mut data);
    println!("{:?}", data);
}

fn main() {
    slices();
}

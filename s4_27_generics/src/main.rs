struct Point<T>
{
    x: T,
    y: T
}

struct PointAlt<T,V>
{
    x: T,
    y: V
}

struct Line<T>
{
    start: Point<T>,
    end: Point<T>
}

fn generics()
{
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 1.2, y: 3.4 };
    // let c: Point<f64,i32> = PointAlt { x: 1.2, y: 3.4 };
    let c = PointAlt { x: 1.2, y: 3 };

    // Won't compile as type(b) != type(a).
    // let myline = Line { start: a, end: b };
}

fn main()
{
    generics();
}

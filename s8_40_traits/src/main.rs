
trait Animal
{
    // Static function that creates an instance.
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human
{
    name: &'static str
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human
    {
        Human{ name: name }
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello", self.name());
    }
}

struct Cat
{
    name: &'static str
}

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat
    {
        Cat{ name: name }
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says meow", self.name());
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self { result += *x; }
        return result;
    }
}

fn traits()
{
    let h = Human { name: "John" };
    h.talk();

    let c = Cat{ name: "Misty" };
    c.talk();

    // Invoke the static factory method.
    let h2 = Human::create("John2");
    h2.talk();

    // Use the static factory from the animal trait.
    let h3:Human = Animal::create("John3");
    h3.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}

fn main() {
    traits();
}

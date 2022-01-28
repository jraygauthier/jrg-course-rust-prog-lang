use std::fs::Permissions;

struct Person
{
    name: String
}

impl Person
{
    fn get_ref_name(&self) -> &String
    // rustc actually implicitely performs lifetime elision here.
    // The signature of this method actually is:
    // fn get_ref_name<'a>(&'a self) -> &'a String
    {
        &self.name
    }
}

struct Company<'z>
{
    name: String,
    // rustc nicely fails with a "missing lifetime specifier" here.
    // This is because create a company taking its ceo person instance by
    // arbitrary reference, we cannot be sure the person instance will
    // outlive its company.
    // ceo: &Person

    // It is however possible to declare to rustc via a `z` lifetime that a
    // `Company` and its `ceo` `Person` should be bound to the same lifetime.
    ceo: &'z Person
}


fn main() {
    // `static` defines a lifetime. In this cause it says the
    // `str` will live as long as the program.
    // `&'static str`

    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss};

    let mut z: &String;
    {
        let p = Person { name: String::from("John") };
        // Note that rustc won't allow this and fails with "`p` does not live
        // long enough". This is because `get_ref_name`'s rustc implicitely
        // bounnd return type's lifetime to that of the instance.
        // z = p.get_ref_name();
    }

    // println!("z = {}", z);
}

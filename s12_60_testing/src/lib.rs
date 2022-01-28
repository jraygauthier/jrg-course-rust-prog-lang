fn my_private() -> u32 { 10 }

pub mod greetings
{
    pub mod english
    {
        pub fn hello() -> String { "hello".to_string() }
        pub fn goodbye() -> String { "goodbye".to_string() }

        fn my_private_in_mod() -> u32 { 5 }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn my_private_in_mod_ok()
            {
                assert_eq!(5, my_private_in_mod());
            }
        }
    }
}


// Only needs the test attribute for `$ cargo test` to see this as a test.
#[test]
fn english_greeting_correct()
{
    assert_eq!("hello", greetings::english::hello());
}

#[test]
// Use this `should_panic` decorator to specify this test
// should panic / fail.
#[should_panic]
fn english_greeting_correct_should_panic()
{
    assert_eq!("not hello", greetings::english::hello());
}

#[test]
// And the `ignore` decorator to ignore the test.
#[ignore = "Unstable"]
fn english_greeting_correct_ignored()
{
    assert_eq!("hello", greetings::english::hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_private_ok()
    {
        assert_eq!(10, my_private());
    }

    #[test]
    fn my_private_in_mod_ok()
    {
        // Not possible to test this one.
        // assert_eq!(10, my_private_in_mod());
    }
}

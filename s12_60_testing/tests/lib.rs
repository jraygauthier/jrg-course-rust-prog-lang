use s12_60_testing::greetings;

// This is our *integration* test module. Any module under the `tests`
// folder will be treated by `cargo` as its own create. No need for
// the `#[cfg(test)]` annotation. Note that it won't be possible
// to test private functions here.

#[test]
fn english_greeting_correct()
{
    assert_eq!("hello", greetings::english::hello());
}

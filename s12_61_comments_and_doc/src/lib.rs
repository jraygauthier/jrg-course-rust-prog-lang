//! This module contains English phrases.
//!
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}!",
//!     s12_61_comments_and_doc::greetings::hello(),
//!     username);
//! ```


pub mod greetings
{
    // short comment

    /*
        Long comment
        with multiple lines.
    */

    /// Applies to code that follows it.
    /// In this case, it's our `hello()` function.
    pub fn hello() -> String { "hello".to_string() /* inline comment*/ }

    /*
        Doc for this module can then be generated using:
        ```bash
        $ cargo doc
        ```

        and previewed using:

        ```bash
        $ firefox ./target/doc/s12_61_comments_and_doc/index.html
        ```

        or alternatively with proper vscode extension installed, the vscode
        command "Rust: Doc Viewer" can be used.
    */
}

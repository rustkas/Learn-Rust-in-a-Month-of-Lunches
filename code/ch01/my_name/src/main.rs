#[allow(unused_variables)]

/// Rust programs start with fn main().
/// You put the code inside a block. It starts with { and ends with }.
fn main() {
    let mut my_name: String = "Dave".to_string();
    my_name.push('!');
    println!("{}", my_name);

    let my_number = 9;

    let some_number = 100; // We can write as much as we want here and the compiler won't look at it

    let some_number/*: i16*/ = 100;

    let some_number = 100; // Let me tell you
                           // a little about this number.
                           // It's 100, which is my favorite number.
                           // It's called some_number but actually I think that...
    let some_number = 100; /* Let me tell you
                           a little about this number.
                           It's 100, which is my favorite number.
                           It's called some_number but actually I think that... */
}

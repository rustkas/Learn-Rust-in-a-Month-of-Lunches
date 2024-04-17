#[allow(dead_code)]
pub fn ex01() {
    use std::borrow::Cow;

    #[derive(Debug)]
    struct ErrorInfo {
        error: LocalError,
        message: String,
    }
    #[derive(Debug)]
    enum LocalError {
        TooBig,
        TooSmall,
    }
    fn generate_message(message: &'static str, error_info: Option<ErrorInfo>) -> Cow<'static, str> {
        match error_info {
            None => message.into(),
            Some(info) => format!("{message}: {info:?}").into(),
        }
    }

    let msg1 = generate_message("Everything is fine", None);
    let msg2 = generate_message(
        "Got an error",
        Some(ErrorInfo {
            error: LocalError::TooBig,
            message: "It was too big".to_string(),
        }),
    );
    for msg in [msg1, msg2] {
        match msg {
            Cow::Borrowed(msg) => {
                println!("Borrowed, didn't need an allocation:\n\r {msg}")
            }
            Cow::Owned(msg) => {
                println!("Owned, because we needed an allocation:\n\r {msg}")
            }
        }
    }
}

pub fn extra_ex01() {
    use std::borrow::Cow;

    fn process_text(data: &str) -> Cow<str> {
        if data.contains("example") {
            let mut owned_string = data.to_owned();
            owned_string.push_str(" with Cow");
            Cow::Owned(owned_string)
        } else {
            Cow::Borrowed(data)
        }
    }

    let input_str = "This is a test";
    let cow = process_text(input_str);
    match cow {
        Cow::Borrowed(b) => println!("Borrowed: {}", b),
        Cow::Owned(o) => println!("Owned: {}", o),
    }
    println!();

    let modified_input = "This is an example string";
    let modified_cow = process_text(modified_input);
    match modified_cow {
        Cow::Borrowed(b) => println!("Borrowed: {}", b),
        Cow::Owned(o) => println!("Owned: {}", o),
    }
    println!();
}

#[allow(dead_code, unused_variables)]
pub fn ex02() {
    use std::borrow::Cow;
    struct User {
        name: Cow<'static, str>,
    }

    let user_name = "User1";
    let other_user_name = "User10".to_string();
    let user1 = User {
        name: user_name.into(),
    };
    let user2 = User {
        name: other_user_name.into(),
    };
    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("{name} - Borrowed name, didn't need an allocation:\n {n}")
            }
            Cow::Owned(n) => {
                println!(
                    "{string} - Owned name because we needed an allocation:\n {n}",
                    string = n.as_str()
                )
            }
        }
    }
}

pub fn ex03() {
    use std::borrow::Cow;
    struct User<'a> {
        name: Cow<'a, str>,
    }

    let user_name = "User1";
    let other_user_name = &"User10".to_string();
    let user1 = User {
        name: user_name.into(),
    };
    let user2 = User {
        name: other_user_name.into(),
    };
    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("Borrowed name, didn't need an allocation:\n {n}")
            }
            Cow::Owned(n) => {
                println!("Owned name because we needed an allocation:\n {n}")
            }
        }
    }
}
fn main() {
    ex03();
}

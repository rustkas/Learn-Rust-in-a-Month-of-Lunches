#![allow(dead_code)]

pub fn ex01() {
    fn just_takes_a_variable<T>(_item: T) {}

    let my_number = 1;
    just_takes_a_variable(my_number);
    just_takes_a_variable(my_number);
    let my_box = Box::new(1);
    just_takes_a_variable(my_box.clone());
    just_takes_a_variable(my_box);
}

pub fn ex02() {
    let my_box = Box::new(1);
    let an_integer = *my_box;
    println!("{an_integer}");
}

pub fn ex03() {
    // struct Holder {
    //     next_holder: Option<Holder>,
    // }

    // let x = Holder {
    //     next_holder: Some(Holder {
    //         next_holder: Some(Holder { next_holder: None }),
    //     }),
    // };
}

pub fn ex04() {
    #[derive(Debug)]
    struct Holder {
        next_holder: Option<Box<Holder>>,
    }

    let x = Holder {
        next_holder: Some(Box::new(Holder {
            next_holder: Some(Box::new(Holder { next_holder: None })),
        })),
    };
    println!("{x:#?}");
}

pub fn ex05() {
    use std::fmt::Display;
    struct DoesntImplementDisplay {}
    fn displays_it<T: Display>(input: T) {
        println!("{}", input);
    }

    let _item: DoesntImplementDisplay = DoesntImplementDisplay {};
    // displays_it(_item);
    displays_it("hello");
}

pub fn ex06() {
    use std::mem::size_of;
    trait JustATrait {}
    enum EnumOfNumbers {
        I8(i8),
        AnotherI8(i8),
        OneMoreI8(i8),
    }

    impl JustATrait for EnumOfNumbers {}

    struct StructOfNumbers {
        an_i8: i8,
        another_i8: i8,
        one_more_i8: i8,
    }
    impl JustATrait for StructOfNumbers {}

    enum EnumOfOtherTypes {
        I8(i8),
        AnotherI8(i8),
        Collection(Vec<String>),
    }
    impl JustATrait for EnumOfOtherTypes {}

    struct StructOfOtherTypes {
        an_i8: i8,
        another_i8: i8,
        a_collection: Vec<String>,
    }
    impl JustATrait for StructOfOtherTypes {}

    struct ArrayAndI8 {
        array: [i8; 1000],
        an_i8: i8,
        in_u8: u8,
    }
    impl JustATrait for ArrayAndI8 {}

    println!(
        "{}, {}, {}, {}, {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTypes>(),
        size_of::<ArrayAndI8>(),
    );

    // fn returns_just_a_trait() -> JustATrait {
    //     let some_enum = EnumOfNumbers::I8(8);
    //     some_enum
    // }

    fn returns_just_a_trait() -> Box<dyn JustATrait> {
        let some_enum = EnumOfNumbers::I8(8);
        Box::new(some_enum)
    }
    // println!("{:?}", returns_just_a_trait())
}

pub fn ex07() {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct ErrorOne;
    impl Error for ErrorOne {}

    impl fmt::Display for ErrorOne {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "You got the first error!")
        }
    }

    #[derive(Debug)]
    struct ErrorTwo;
    impl Error for ErrorTwo {}
    impl fmt::Display for ErrorTwo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "You got the second error!")
        }
    }

    fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
        match input {
            0 => Err(Box::new(ErrorOne)),
            1 => Err(Box::new(ErrorTwo)),
            _ => Ok("Looks fine to me".to_string()),
        }
    }

    let vec_of_u8s = vec![0_u8, 1, 80];
    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{message}"),
        }
    }
}

pub fn ex08() {
    fn handle_error_inside_function() {
        println!("{:?}", "seven".parse::<i32>());
    }

    handle_error_inside_function();
}

pub fn ex09() {
    use std::error::Error;
    fn parse_numbers(int: &str, float: &str) -> Result<f64, Box<dyn Error>> {
        let num_1 = int.parse::<i32>()?;
        let num_2 = float.parse::<f64>()?;
        Ok(num_1 as f64 + num_2)
    }

    let _my_number = parse_numbers("8", "ninepointnine").unwrap();
}

pub fn ex10() {
    use std::fmt;
    enum MyError {
        TooMuchStuff,
        CantConnect,
        NoUserRegistered,
        SomethingElse,
    }

    impl std::error::Error for MyError {}
    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            write!(f, "Wouldn't you like to know...")
        }
    }

    impl fmt::Debug for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Lol not telling you what went wrong")
                .finish()
        }
    }

    let err = MyError::TooMuchStuff;
    println!("{err}");
    println!("{err:?}");
}

pub fn ex11() {
    use std::error::Error;
    use std::fmt;
    use std::sync::mpsc::RecvError;
    enum MyError {
        TooMuchStuff,
        CantConnect,
        NoUserRegistered,
        SomethingElse,
    }
    impl std::error::Error for MyError {}

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            write!(f, "Wouldn't you like to know...")
        }
    }
    impl fmt::Debug for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Lol not telling you what went wrong")
                .finish()
        }
    }

    fn give_error_back(is_true: bool) -> Box<dyn Error> {
        if is_true {
            Box::new(MyError::TooMuchStuff)
        } else {
            Box::new(RecvError)
        }
    }

    let errs = [true, false, false, true]
        .into_iter()
        .map(|boolean| give_error_back(boolean))
        .collect::<Vec<_>>();

    println!("{errs:#?}");
    for err in errs.iter() {
        if let Some(_my_error) = err.downcast_ref::<MyError>() {
            println!("Got a MyError!");
        } else if let Some(_parse_error) = err.downcast_ref::<RecvError>() {
            println!("Got a RecvError!");
        }
    }
}
fn main() {
    ex11();
}

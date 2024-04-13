mod util;

pub fn ex01() {
    fn check_error() -> Result<(), ()> {
        Ok(())
    }
    let _ = check_error();
}

pub fn ex02() {
    fn see_if_number_is_even(input: i32) -> Result<(), ()> {
        if input % 2 == 0 {
            return Ok(());
        } else {
            return Err(());
        }
    }

    if see_if_number_is_even(5).is_ok() {
        println!("It's okay, guys")
    } else {
        println!("It's an error, guys")
    }
}

pub fn ex03() {
    fn check_if_five(number: i32) -> Result<i32, String> {
        match number {
            5 => Ok(number),
            _ => Err(format!("Sorry, bad number. Expected: 5 Got: {number}")),
        }
    }

    for number in 4..=7 {
        println!("{:?}", check_if_five(number));
    }
}

pub fn ex04() {
    let error_value: Result<i32, &str> = Err("There was an error");
    error_value.unwrap();
}

pub fn ex05() {
    #[derive(Debug)]
    struct FromUtf8Error {
        message: String,
    }

    impl FromUtf8Error {
        fn new(message: &str) -> Self {
            FromUtf8Error {
                message: message.to_string(),
            }
        }
    }

    use std::error::Error;
    use std::fmt::Display;
    use std::fmt::Formatter;

    impl Display for FromUtf8Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "FromUtf8Error: {}", self.message)
        }
    }

    impl Error for FromUtf8Error {}

    pub fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error> {
        match String::from_utf8(vec) {
            Ok(s) => Ok(s),
            Err(_) => Err(FromUtf8Error::new("Invalid UTF-8 sequence")),
        }
    }

    let valid_utf8 = vec![104, 101, 108, 108, 111];
    let invalid_utf8 = vec![240, 144, 128]; // Недопустимая последовательность UTF-8

    match from_utf8(valid_utf8) {
        Ok(s) => println!("Valid UTF-8 string: {s}"),
        Err(e) => println!("Error converting from UTF-8: {e}"),
    }

    match from_utf8(invalid_utf8) {
        Ok(s) => println!("Valid UTF-8 string: {s}"),
        Err(e) => println!("Error converting from UTF-8: {e}"),
    }
}

pub fn ex06() {
    let array = [2, 3, 4];
    let get_one = array.get(0);
    let get_two = array.get(10);
    println!("{:?}", get_one);
    println!("{:?}", get_two);
}

pub fn ex07() {
    let array = [2, 3, 4];
    for index in 0..10 {
        match array.get(index) {
            Some(number) => println!("The number is: {number}"),
            None => {}
        }
    }
}

pub fn ex08() {
    let array = [2, 3, 4];
    for index in 0..10 {
        if let Some(number) = array.get(index) {
            println!("The number is: {number}");
        }
    }
}

pub fn ex09() {
    let array = [2, 3, 4];
    for index in 0..=10 {
        if let Some(number) = array.get(index) {
            println!("The number is: {number}");
        }
        let Some(number) = array.get(index) else {
            continue;
        };
        println!("The number is: {number}");
    }
}

// let else
pub fn ex10() {
    let array = [2, 3, 4];
    for index in 0..=10 {
        let Some(number) = array.get(index) else {
            println!(
                "Looks like we got a {result:?}!",
                result = Option::<i32>::None
            );
            println!("We can still do whatever we want inside this block");
            println!("We just have to end with 'diverging code'");
            print!("Because after this block, ");
            println!("the variable 'number' has to exist");
            println!("Time to break the loop now, bye");
            break;
            // return ();
        };
        println!("The number is: {number}");
    }
}

// while let
pub fn ex11() {
    let weather_vec = [
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];
    for mut city in weather_vec {
        println!("For the city of {}:", city[0]);
        println!("The humidity is {}%:", city.pop().unwrap());
        
        let mut data:Vec<&str> = city[1..=city.len()-1].into();
        while let Some(information) = data.pop() {
            if let Ok(number) = information.parse::<i32>() {
                println!("The temperature is: {number}");
            } else {
                let weather = information;
                println!("The weather is: {weather}");
            }
        }
        println!("");
    }
}

fn main() {
    ex11();
}

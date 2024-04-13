pub fn ex01() {
    use std::num::ParseIntError;

    fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
        let parsed_number = input.parse::<i32>()?;
        println!("Number parsed successfully into {parsed_number}");
        Ok(parsed_number)
    }
    match parse_and_log_str("1") {
        Ok(result) => println!("Pased number is {result}"),
        Err(err) => println!("{err}",),
    };
    match parse_and_log_str("one") {
        Ok(result) => println!("Pased number is {result}"),
        Err(err) => println!("{err}",),
    };
}

pub fn ex02() {
    use std::num::ParseIntError;

    fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
        let parsed_number = match input.parse::<i32>() {
            Ok(number) => number,
            Err(e) => return Err(e),
        };
        println!("Number parsed successfully into {parsed_number}");
        Ok(parsed_number)
    }
    match parse_and_log_str("1") {
        Ok(result) => println!("Pased number is {result}"),
        Err(err) => println!("{err}",),
    };
    match parse_and_log_str("one") {
        Ok(result) => println!("Pased number is {result}"),
        Err(err) => println!("{err}",),
    };
}

pub fn ex03() {
    use std::num::ParseIntError;
    fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
        let parsed_number = input.parse::<i32>()?;
        println!("Number parsed successfully into {parsed_number}");
        Ok(parsed_number)
    }

    let str_vec = ["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_and_log_str(item);
        println!("Result: {parsed:?}");
    }
}

pub fn ex04() {
    use std::num::ParseIntError;

    fn parse_str(input: &str) -> Result<i32, ParseIntError> {
        let parsed_number = input
            .parse::<u16>()?
            .to_string()
            .parse::<u32>()?
            .to_string()
            .parse::<i32>()?;
        println!("Number parsed successfully into {parsed_number}");
        Ok(parsed_number)
    }

    let str_vec = ["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }
}

pub fn ex05() {
    fn turn_into_string_and_parse(bytes: Vec<u8>) -> i32 {
        let as_string = String::from_utf8(bytes).unwrap();
        let as_num = as_string.parse::<i32>().unwrap();
        as_num
    }

    let num = turn_into_string_and_parse(vec![49, 53, 53]);
    println!("{num}");
}

pub fn ex06() {
    // use std::num::ParseIntError;
    //     use std::string::FromUtf8Error;
    //     fn turn_into_string_and_parse(bytes: Vec<u8>) ->Result<i32,_> {
    // let num = String::from_utf8(bytes)?.parse::<i32>()?;
    // Ok(num)
    // }
}

pub fn ex07() {
    panic!();
}

pub fn ex08() {
    panic!("Time to panic!");
}

pub fn ex09() {
    fn print_all_three_things(vector: Vec<i32>) {
        println!("{}, {}, {}", vector[0], vector[1], vector[2]);
    }

    let my_vec = vec![8, 9, 10];
    print_all_three_things(my_vec);
}
pub fn ex10() {
    fn print_all_three_things(vector: Vec<i32>) {
        println!("{}, {}, {}", vector[0], vector[1], vector[2]);
    }

    let my_vec = vec![8, 9, 10, 10, 55, 99];
    print_all_three_things(my_vec);
}

pub fn ex11() {
    fn print_all_three_things(vector: Vec<i32>) {
        if vector.len() != 3 {
            panic!("my_vec must always have three items");
        }
        println!("{}, {}, {}", vector[0], vector[1], vector[2]);
    }

    let my_vec = vec![8, 9, 10, 10, 55, 99];
    print_all_three_things(my_vec);
}

pub fn ex12() {
    let my_name = "Loki Laufeyson";
    assert!(my_name == "Loki Laufeyson");
    assert_eq!(my_name, "Loki Laufeyson");
    assert_ne!(my_name, "Mithridates");
}

pub fn ex13() {
    let my_name = "Loki Laufeyson";
    assert!(
        my_name == "Loki Laufeyson",
        "Name {my_name} is wrong: should be Loki Laufeyson"
    );
    assert_eq!(
        my_name, "Loki Laufeyson",
        "{my_name} and Loki Laufeyson should be equal"
    );
    assert_ne!(
        my_name, "Mithridates",
        "You entered {my_name}. Input must not equal Mithridates"
    );
}

pub fn ex14() {
    let my_name = "Mithridates";
    assert_ne!(
        my_name, "Mithridates",
        "You entered {my_name}. Input must not equal Mithridates"
    );
}

pub fn ex15() {
    fn get_fourth(input: &Vec<i32>) -> i32 {
        let fourth = input.get(3).unwrap();
        *fourth
    }
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);
    println!("{fourth}");
}

pub fn ex16() {
    fn get_fourth(input: &Vec<i32>) -> i32 {
        let fourth = input.get(3).expect("Input vector needs at least 4 items");
        *fourth
    }
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);
    println!("{fourth}");
}

// here is an example of a bad practice: a function that tries to unwrap two
// times. It takes a Vec<Option<i32>>, so maybe each part will have a Some<i32> or
// maybe a None:
pub fn ex17() {
    fn try_two_unwraps(input: Vec<Option<i32>>) {
        println!("Index 0 is: {}", input[0].unwrap());
        println!("Index 1 is: {}", input[1].unwrap());
    }
    let vector = vec![None, Some(1000)];
    try_two_unwraps(vector);
}
// expect is a liitle bit better
pub fn ex18() {
    fn try_two_unwraps(input: Vec<Option<i32>>) {
        println!(
            "Index 0 is: {}",
            input[0].expect("The first unwrap had a None!")
        );
        println!(
            "Index 1 is: {}",
            input[1].expect("The second unwrap had a None!")
        );
    }
    let vector = vec![None, Some(1000)];
    try_two_unwraps(vector);
}
pub fn ex19() {
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or(&0);
    let fourth_value = *fourth;
    println!("{fourth_value}");
}
fn main() {
    ex19();
}

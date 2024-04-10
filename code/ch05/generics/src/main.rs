mod util;

pub fn ex01() {
    fn return_item(item: i32) -> i32 {
        println!("Here is your item.");
        item
    }
    let item = return_item(5);
    println!(
        "{variable_name} = {item:?}",
        variable_name = stringify!(item)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(item), type = crate::util::get_type(&item));
}

pub fn ex02() {
    fn return_i32(number: i32) -> i32 {
        number
    }
    fn return_i16(number: i16) -> i16 {
        number
    }
    fn return_u8(number: u8) -> u8 {
        number
    }

    _ = return_i32(0);
    _ = return_i16(0);
    _ = return_u8(0);
}

pub fn ex03() {
    fn return_item<T>(item: T) -> T {
        println!("Here is your item.");
        item
    }

    fn print(item: &Number) {
        println!("Type of {variable_name} = {type}", variable_name = stringify!(item), type = crate::util::get_type(&item));
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Number {
        U8(u8),
        I16(i16),
        I32(i32),
    }
    use Number::*;
    let array = [U8(0), I16(0), I32(0)];
    for item in array {
        print(&item);
        match &item {
            U8(value) => {
                let _ = return_item(value);
            }
            I16(value) => {
                let _ = return_item(value);
            }
            I32(value) => {
                let _ = return_item(value);
            }
        }
    }
}

pub fn ex04() {
    fn return_item<MyType>(item: MyType) -> MyType {
        println!("Here is your item.");
        item
    }

    let _ = return_item(5);
}

pub fn ex05() {
    use std::fmt::Debug;
    fn return_item<T: Debug>(item: T) -> T {
        println!("Here is your item. {item:?}");
        item
    }

    let _ = return_item(5);
}

pub fn ex06() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Animal {
        name: String,
        age: u8,
    }
    fn print_item<T: std::fmt::Debug>(item: T) {
        println!("Here is your item: {item:?}");
    }

    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };
    let number = 55;
    print_item(charlie);
    print_item(number);
}

pub fn ex07() {
    use std::fmt::Display;
    fn compare_and_display<T: Display, U: Display + PartialOrd>(
        statement: T,
        input_1: U,
        input_2: U,
    ) {
        println!(
            "{statement}! Is {input_1} greater than {input_2}? {}",
            input_1 > input_2
        );
    }

    compare_and_display("Listen up!", 9, 8);
}

pub fn ex08() {
    use std::fmt::Display;
    fn compare_and_display<T, U>(statement: T, input_1: U, input_2: U) 
        where 
            T: Display, 
            U: Display + PartialOrd
        {
            println!(
                "{statement}! Is {input_1} greater than {input_2}? {}",
                input_1 > input_2
            );
    }

    compare_and_display("Listen up!", 9, 8);
}

pub fn ex09() {
    use std::fmt::Display;

    fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
        println!("I have two things to say: {statement_1} and {statement_2}");
    }
    
    say_two("Hello there!", String::from("I hate sand."));
    say_two(String::from("Where is Padme?"), String::from("Is she all right?"));
}
fn main() {
    ex09();
}

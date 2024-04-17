#![allow(unused_variables)]
#![allow(dead_code)]

pub fn ex01() {
    struct JustAStruct {}

    let some_char = 'ん';
    println!("{some_char}");
}

pub fn ex02() {
    struct Struct1 {}
    struct Struct2 {}
    struct Struct3 {}
    struct Struct4 {}
    struct Struct5 {}

    let char1 = 'ん';
    let char2 = ';';
    let some_str = "I'm just a regular &str";
    let some_vec = vec!["I", "am", "just", "a", "vec"];
}

pub fn ex03() {
    #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
    struct HoldsAString {
        the_string: String,
    }
    let my_string = HoldsAString {
        the_string: "Here I am!".to_string(),
    };
    println!("{:?}", my_string);
}

pub fn ex04() {
    #[derive(Clone, Copy)]
    struct NumberAndBool {
        number: i32,
        true_or_false: bool,
    }
    fn does_nothing(input: NumberAndBool) {}

    let number_and_bool = NumberAndBool {
        number: 8,
        true_or_false: true,
    };
    does_nothing(number_and_bool);
    does_nothing(number_and_bool);
}
#[allow(deprecated)]
pub fn ex05() {
    #[deprecated]
    fn deprecated_function() {}

    deprecated_function();
}
fn main() {
    ex04();
}

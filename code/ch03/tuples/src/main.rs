#[macro_use]
mod util;

pub fn ex01() {
    fn do_something() {}
    let result = do_something();
    println!("Type of {variable_name} = {type}", variable_name = stringify!(result), type = crate::util::get_type(&result));
}

pub fn ex02() {
    fn do_something() -> () {}
    let result = do_something();
    println!("Type of {variable_name} = {type}", variable_name = stringify!(result), type = crate::util::get_type(&result));
}

pub fn ex03() {
    fn just_makes_an_i32() {
        let _unused_number = 10;
    }
    let result = just_makes_an_i32();
    println!("Type of {variable_name} = {type}", variable_name = stringify!(result), type = crate::util::get_type(&result));
}

pub fn ex04() {
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    )
}
pub fn ex05() {
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "{variable_name} = {random_tuple:?}",
        variable_name = stringify!(random_tuple)
    );

    println!("Type of {variable_name} = {type}", variable_name = stringify!(random_tuple), type = crate::util::get_type(&random_tuple));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(random_tuple.0), type = crate::util::get_type(&random_tuple.0));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(random_tuple.1), type = crate::util::get_type(&random_tuple.1));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(random_tuple.2), type = crate::util::get_type(&random_tuple.2));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(random_tuple.3), type = crate::util::get_type(&random_tuple.3));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(random_tuple.4), type = crate::util::get_type(&random_tuple.4));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(random_tuple.5), type = crate::util::get_type(&random_tuple.5));
}

pub fn ex06() {
    let strings = ("one".to_string(), "two".to_string(), "three".to_string());
    println!(
        "{variable_name} = {strings:?}",
        variable_name = stringify!(strings)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(strings), type = crate::util::get_type(&strings));
}

fn print_info<T>(a: &T)
where
    T: std::fmt::Debug,
{
    println!("{variable_name} = {a:?}", variable_name = stringify!(a));
}
pub fn ex07() {
    let strings = ("one".to_string(), "two".to_string(), "three".to_string());
    let (a, b, c) = strings.clone();
    print_info(&strings);
    print_info(&a);
    print_info(&b);
    print_info(&c);
    println!(
        "{variable_name} = {strings:?}",
        variable_name = stringify!(strings)
    );
    println!("{variable_name} = {a:?}", variable_name = stringify!(a));
    println!("{variable_name} = {b:?}", variable_name = stringify!(b));
    println!("{variable_name} = {c:?}", variable_name = stringify!(c));

    println!("Type of {variable_name} = {type}", variable_name = stringify!(strings), type = crate::util::get_type(&strings));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(a), type = crate::util::get_type(&a));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(b), type = crate::util::get_type(&b));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(c), type = crate::util::get_type(&c));
}

pub fn ex08() {
    let strings = ("one", "two", "three");
    let (a, b, c) = strings.clone();

    println!(
        "{variable_name} = {strings:?}",
        variable_name = stringify!(strings)
    );
    println!("{variable_name} = {a:?}", variable_name = stringify!(a));
    println!("{variable_name} = {b:?}", variable_name = stringify!(b));
    println!("{variable_name} = {c:?}", variable_name = stringify!(c));

    println!("Type of {variable_name} = {type}", variable_name = stringify!(strings), type = crate::util::get_type(&strings));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(a), type = crate::util::get_type(&a));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(b), type = crate::util::get_type(&b));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(c), type = crate::util::get_type(&c));
}

pub fn ex10() {
    let strings = ("one", "two", "three");
    let (_, b, c) = strings;

    println!(
        "{variable_name} = {strings:?}",
        variable_name = stringify!(strings)
    );
    // println!("{variable_name} = {a:?}", variable_name = stringify!(a));
    println!("{variable_name} = {b:?}", variable_name = stringify!(b));
    println!("{variable_name} = {c:?}", variable_name = stringify!(c));

    println!("Type of {variable_name} = {type}", variable_name = stringify!(strings), type = crate::util::get_type(&strings));
    // println!("Type of {variable_name} = {type}", variable_name = stringify!(a), type = crate::util::get_type(&a));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(b), type = crate::util::get_type(&b));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(c), type = crate::util::get_type(&c));
}

fn main() {
    ex10()
}

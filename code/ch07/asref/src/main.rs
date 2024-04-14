pub fn ex01() {
    use std::fmt::Display;
    fn print_it<T: Display>(input: T) {
        println!("{input}");
    }

    print_it("Please print me");
    print_it(0 as u32);
    print_it(0 as f32);
}

pub fn ex02() {
    fn print_it<T: AsRef<str>>(input: T) {
        println!("{}", input.as_ref())
    }

    print_it("Please print me");
    print_it("Also, please print me".to_string());
    // print_it(7);
}
#[allow(dead_code)]
pub fn ex03() {
    fn print_it<T: AsRef<i32>>(input: T) {
        println!("{}", input.as_ref())
    }

    // print_it(7 as i32);
}

fn main() {
    ex02();
}

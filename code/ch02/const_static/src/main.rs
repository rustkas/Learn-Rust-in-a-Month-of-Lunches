pub const NUMBER_OF_MONTHS: u32 = 12;
pub static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
fn print_months() {
    println!("Number of months in the year: {NUMBER_OF_MONTHS}");
}
fn ex01() {
    print_months();
    println!("Seasons: {:?}", SEASONS);
}
fn main() {
    ex01();
}

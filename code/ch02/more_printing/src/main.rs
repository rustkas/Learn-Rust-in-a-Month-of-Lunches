pub fn ex01() {
    print!("\t Start with a tab\nand move to a new line\r\n");
}

pub fn ex02() {
    println!(
        "Inside quotes
    you can write over
    many lines
    and it will print just fine."
    );
    println!(
        "If you forget to write
    on the left side, the spaces
    will be added when you print."
    );
}

pub fn ex03() {
    println!("Here are two escape characters: \\n and \\t");
}

pub fn ex04() {
    let string01 = "He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file.";
    // let string01 = "He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file.";

    println!("{string01}");
    println!(
        r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
    );
}

pub fn ex05() {
    let my_string = "'Ice to see you,' he said.";
    let quote_string = r#""Ice to see you," he said."#;
    let hashtag_string = r##"The hashtag "#IceToSeeYou" had become very popular."##;
    let many_hashtags =
        r####""You don't have to type "###" to use a hashtag. You can just use #."####;
    // println!(
    //     "{}\n{}\n{}\n{}\n",
    //     my_string, quote_string, hashtag_string, many_hashtags
    // );

    println!("{my_string}\n{quote_string}\n{hashtag_string}\n{many_hashtags}\n");
}

pub fn ex06() {
    println!("{:?}", b"This will look like numbers");
}
pub fn ex07() {
    println!("{:?}", br##"I like to write "#"."##);
}
pub fn ex08() {
    let char01 = '행';
    let char02 = 'H';
    let char03 = '居';
    let char04 = 'い';
    let array = [char01, char02, char03, char04];
    for item in array {
        println!("{:X}", item as u32);
    }

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");
}

pub fn ex09() {
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);
}

pub fn ex10() {
    let number = 555;
    println!(
        "Binary: {:b}, hexadecimal: {:x}, octal: {:o}",
        number, number, number
    );
}

pub fn ex11() {
    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!(
        "This is {1} {2}, son of {0} {2}.",
        father_name, son_name, family_name
    );
}
pub fn ex12() {
    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );
}

pub fn ex13() {
    let value: u64 = 555555555;
    println!("{value:0^11}");
    println!("{value:0<11}");
    println!("{value:0>11}");
}

pub fn ex14() {
    let letter = "a";
    println!("{:ㅎ^11}", letter);
}

pub fn ex15() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
fn main() {
    ex15();
}

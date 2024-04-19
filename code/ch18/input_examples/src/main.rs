pub fn ex01() {
    use std::io;
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {input_string}");
        println!("result={}", input_string != "x");
    }
    println!("See you later!");
}

pub fn ex02() {
    use std::env::args;

    let input = args();
    for entry in input {
        println!("You entered: {}", entry);
    }
}

pub fn ex03() {
    use std::env::args;

    let input = args();
    input.skip(1).for_each(|item| {
        println!(
            "You wrote {item}, which in capital letters is {}",
            item.to_uppercase()
        );
    });
}

use std::env;
const DEV_URL: &str = "www.somedevurl.com";
const PROD_URL: &str = "www.someprodurl.com";

pub fn ex04() {
    match std::env::var("RUST_LOG") {
        Ok(log) => println!("Logging at {log} level"),
        Err(_) => match std::env::var("LOGGER_URL") {
            Ok(url) if url == DEV_URL => {
                println!("Dev url indicated, defaulting to debug");
                env::set_var("RUST_LOG", "DEBUG");
            }
            Ok(url) if url == PROD_URL => {
                println!("Prod url indicated, defaulting to info");
                env::set_var("RUST_LOG", "INFO");
            }
            _ => {
                println!("No valid url indicated, defaulting to info");
                env::set_var("RUST_LOG", "INFO");
            }
        },
    }
}

pub fn ex05()-> std::io::Result<()> {
    use std::fs;
    use std::io::*;
  

    let mut file = fs::File::create("myfilename.txt")?; // Теперь `?` может быть использован
    file.write_all(b"Let's put this in the file")?;

    fs::remove_file("myfilename.txt")?;

    Ok(())
}

fn main() {
    let _ = ex05();
}

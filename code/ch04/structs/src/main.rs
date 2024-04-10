mod util;

pub fn ex01() {
    #[derive(Debug)]
    struct FileDirectory;
    let file_dir: FileDirectory;
    file_dir = FileDirectory {};
    println!(
        "{variable_name} = {file_dir:?}",
        variable_name = stringify!(file_dir)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(file_dir), type = crate::util::get_type(&file_dir));
}

pub fn ex02() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct ColorRgb(u8, u8, u8);

    let color: ColorRgb;
    color = ColorRgb(50, 0, 50);
    println!(
        "{variable_name} = {color:?}",
        variable_name = stringify!(color)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(color), type = crate::util::get_type(&color));
}

pub fn ex03() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct ColorRgb(u8, u8, u8);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct SizeAndColor {
        size: u32,
        color: ColorRgb,
    }

    let my_color = ColorRgb(50, 0, 50);
    let size_and_color = SizeAndColor {
        size: 150,
        color: my_color,
    };

    println!(
        "{variable_name} = {size_and_color:?}",
        variable_name = stringify!(size_and_color)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(color), type = crate::util::get_type(&size_and_color));
}

pub fn ex04() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
    }

    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population: population,
        capital: capital,
        leader_name: leader_name,
    };
    println!(
        "{variable_name} = {kalmykia:?}",
        variable_name = stringify!(kalmykia)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(kalmykia), type = crate::util::get_type(&kalmykia));
}

pub fn ex05() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
    }

    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population,
        capital,
        leader_name,
    };
    println!(
        "{variable_name} = {kalmykia:?}",
        variable_name = stringify!(kalmykia)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(kalmykia), type = crate::util::get_type(&kalmykia));
}

pub fn ex06() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
    }

    let kalmykia = Country {
        population: 500_000,
        capital: String::from("Elista"),
        leader_name: String::from("Batu Khasikov"),
    };
    println!(
        "{variable_name} = {kalmykia:?}",
        variable_name = stringify!(kalmykia)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(kalmykia), type = crate::util::get_type(&kalmykia));
}

fn main() {
    ex06();
}

mod util;

pub fn info<T>(my_vec: &Vec<T>)
where
    T: std::fmt::Debug,
{
    println!(
        "{variable_name} = {my_vec:?}",
        variable_name = stringify!(my_vec)
    );
    println!(
        "Capacity of {variable_name} = {capacity}",
        variable_name = stringify!(my_vec),
        capacity = my_vec.capacity()
    );

    println!("Type of {variable_name} = {type}", variable_name = stringify!(my_vec), type = crate::util::get_type(&my_vec));
}

pub fn ex01() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    println!("{my_vec:?}");
    println!("{}", crate::util::get_type(&my_vec));
}

pub fn ex02() {
    let mut vector01: Vec<(i32, i32)>;
    vector01 = Vec::new();
    vector01.push((1, 1));
    println!("{vector01:?}");
    println!("{}", crate::util::get_type(&vector01));
}

pub fn ex03() {
    let mut vector01: Vec<Vec<String>>;
    let inner_vector = {
        let name1 = String::from("Windy");
        let name2 = String::from("Gomesy");
        let mut my_vec = Vec::new();
        my_vec.push(name1);
        my_vec.push(name2);
        my_vec
    };
    vector01 = Vec::new();
    vector01.push(inner_vector);
    println!("{vector01:?}");
    println!("{}", crate::util::get_type(&vector01));
}

pub fn ex04() {
    let mut my_vec = vec![8, 10, 10];

    my_vec.push(-10);
    my_vec.push(-10);
    my_vec.push(-8);
    println!("{my_vec:?}");
    println!("{}", crate::util::get_type(&my_vec));
}

pub fn ex05() {
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("Three to five: {three_to_five:?}");
    println!("start at two: {start_at_two:?}");
    println!("end at five: {end_at_five:?}");
    println!("everything: {everything:?}");
}

pub fn ex06() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!(
        "Capacity of {} = {}",
        stringify!(num_vec),
        num_vec.capacity()
    );
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!(
        "Capacity of {} = {}",
        stringify!(num_vec),
        num_vec.capacity()
    );
    num_vec.push('a');
    println!(
        "Capacity of {} = {}",
        stringify!(num_vec),
        num_vec.capacity()
    );
    println!("{num_vec:?}");
    println!("{}", crate::util::get_type(&num_vec));
}

pub fn ex07() {
    let mut num_vec = Vec::with_capacity(8);
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    println!(
        "Capacity of {} = {}",
        stringify!(num_vec),
        num_vec.capacity()
    );
}

pub fn ex08() {
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into();
    info(&my_vec);
    info(&my_vec2);
}

fn main() {
    ex08();
}

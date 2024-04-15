pub fn ex01() {
    let my_number = 8;
    dbg!(my_number);
}

#[allow(unused_variables, unused_assignments)]
pub fn ex02() {
    let mut my_number = 9;
    my_number += 10;
    let new_vec = vec![8, 9, 10];
    let double_vec = new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
}

#[allow(unused_variables, unused_assignments)]
pub fn ex0201() {
    let mut my_number = dbg!(9);
    dbg!(my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);
}

#[allow(unused_variables, unused_assignments)]
pub fn ex0202() {
    let mut my_number = 9;
    dbg!(my_number);
    dbg!(my_number += 10);
    let new_vec = vec![8, 9, 10];
    let new_vec_clone = dbg!(new_vec.clone());
    let double_vec = new_vec_clone.iter().map(|x| x * 2).collect::<Vec<i32>>();
    dbg!(double_vec);
}

pub fn ex03() {
    let new_vec = vec![8, 9, 10];
    let _double_vec = new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
}

pub fn ex0301() {
    let new_vec = vec![8, 9, 10];
    let _double_vec = new_vec
        .iter()
        .inspect(|first_item| println!("The item is: {first_item}"))
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
}

pub fn ex0302() {
    let new_vec = vec![8, 9, 10];
    let _double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            println!("The item is: {first_item}");
            match **first_item % 2 {
                0 => println!("It is even."),
                _ => println!("It is odd."),
            }
            println!("In binary it is {:b}.", first_item);
        })
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
}
fn main() {
    ex0302();
}

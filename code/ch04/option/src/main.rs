mod util;

pub fn ex01() {
    fn take_fifth_item(value: Vec<i32>) -> i32 {
        value[4]
    }

    let new_vec = vec![1, 2];
    let index = take_fifth_item(new_vec);
    println!("{name} = {value}", name = stringify!(index), value = index);
}

fn main() {
    ex01();
}

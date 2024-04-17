pub fn ex01() {
    fn prints_number(input: i32) {
        assert_eq!(input % 2, 0);
        println!("The number is not odd. It is {input}");
    }
    prints_number(56);
}

pub fn ex02() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    vec.extend([1, 2, 3].iter().copied());
    for x in &vec {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);
}

fn main() {
    ex02();
}

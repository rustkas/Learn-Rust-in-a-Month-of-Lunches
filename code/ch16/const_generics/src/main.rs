#![allow(dead_code)]

pub fn ex01() {
    struct Buffers {
        array_one: [u8; 640],
        array_two: [u8; 640],
    }
    struct BigBuffers {
        array_one: [u8; 1280],
        array_two: [u8; 1280],
    }
}

pub fn ex02() {
    struct Buffers<T, const N: usize> {
        array_one: [T; N],
        array_two: [T; N],
    }
}

pub fn ex03() {
    #[derive(Debug)]
    struct Buffers<T, const N: usize> {
        array_one: [T; N],
        array_two: [T; N],
    }

    let buffer_1 = Buffers {
        array_one: [0u8; 3],
        array_two: [0; 3],
    };
    let buffer_2 = Buffers {
        array_one: [0i32; 4],
        array_two: [10; 4],
    };
    println!("{buffer_1:#?}, {buffer_2:#?}");
}

pub fn ex04() {
    const NUMBER: u8 = give_eight();
    const fn give_eight() -> u8 {
        8
    }

    let mut my_vec = Vec::new();
    my_vec.push(give_eight());
    println!("{my_vec:?}");
}

pub fn ex05() {
    use std::sync::Mutex;
    #[derive(Debug)]
    struct Log {
        date: &'static str,
        message: String,
    }
    static GLOBAL_LOGGER: Mutex<Vec<Log>> = Mutex::new(Vec::new());
    fn add_message(date: &'static str) {
        GLOBAL_LOGGER.lock().unwrap().push(Log {
            date,
            message: "Everything's fine".to_string(),
        });
    }

    add_message("2022-12-12");
    add_message("2023-05-05");
    println!("{GLOBAL_LOGGER:#?}");
}
#[allow(unused_unsafe)]
pub fn ex06() {
    let my_name = unsafe { "My name" };
    println!("{my_name}");
}

pub fn ex07() {
    unsafe fn uh_oh() {}

    unsafe {
        uh_oh();
    }
}

pub fn ex08() {
    static mut NUMBER: u32 = 0;
    unsafe {
        NUMBER += 1;
        println!("{NUMBER}");
    }
}

pub fn ex09() {
    static mut NUMBER: u32 = 0;
    let mut join_handle_vec = vec![];
    for _ in 0..10 {
        join_handle_vec.push(std::thread::spawn(|| {
            for _ in 0..10 {
                unsafe {
                    NUMBER += 1;
                }
            }
        }));
    }
    for handle in join_handle_vec {
        handle.join().unwrap();
    }

    unsafe {
        println!("{NUMBER}");
    }
}

pub fn ex0901() {
    static mut NUMBER: u32 = 0;
    let mut join_handle_vec = vec![];
    for _ in 0..10000 {
        join_handle_vec.push(std::thread::spawn(|| {
            for _ in 0..10000 {
                unsafe {
                    NUMBER += 1;
                }
            }
        }));
    }
    for handle in join_handle_vec {
        handle.join().unwrap();
    }

    unsafe {
        println!("{NUMBER}");
    }
}

pub fn ex10() {
    use std::mem::transmute;
    let x = 19;
    let y = unsafe { transmute::<i32, u32>(x) };
    println!("{y}");
}

pub fn ex1001() {
    use std::mem::transmute;
    let x = -19;
    let y = unsafe { transmute::<i32, u32>(x) };
    println!("{y}");

    println!("{:b}\n{:b}", -19, 4294967277u32);
}
pub fn ex11() {
    struct User {
        name: String,
        number: u32,
    }

    println!("{}", std::mem::size_of::<User>());
}

pub fn ex12() {
    use std::mem::transmute;
    struct User {
        name: String,
        number: u32,
    }

    let some_i32s = [1, 2, 3, 4, 5, 6, 7, 8];
    let _user = unsafe {
        transmute::<[i32; 8], User>(some_i32s);
    };
}

pub fn ex13() {
    let my_option = Some(10);
    // SAFETY: my_option is declared as Some(10). It will never be None
    let unwrapped = unsafe { my_option.unwrap_unchecked() };
    println!("{unwrapped}");
}

pub fn ex14() {
    let my_option:Option<i32> = None;
    // SAFETY: my_option is declared as Some(10). It will never be None
    let unwrapped = unsafe { my_option.unwrap_unchecked() };
    println!("{unwrapped}");
}
fn main() {
    ex14();
}

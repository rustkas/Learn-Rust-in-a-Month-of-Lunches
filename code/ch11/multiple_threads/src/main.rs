pub fn ex01() {
    std::thread::spawn(|| {
        println!("I am printing something");
    });
}
pub fn ex02() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            println!("I am printing something");
        });
    }
}

pub fn ex03() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            println!("I am printing something");
        });
    }
    let mut busy_work = vec![];
    for _ in 0..1_000_000 {
        busy_work.push(9);
        busy_work.pop();
    }
}

pub fn ex04() {
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        let _ = handle.join();
    }
}

pub fn ex05() {
    let mut join_handles = vec![];
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
}

pub fn ex06() {
    let mut join_handles = vec![];
    for num in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("Inside thread number: {num}");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
}

pub fn ex07() {
    let my_string = String::from("I will go into the closure");
    let my_closure = || println!("{my_string}");
    my_closure();
    my_closure();
}

pub fn ex08() {
    let mut my_string = String::from("I will go into the closure");
    let mut my_closure = || {
        my_string.push_str(" now");
        println!("{my_string}");
    };
    my_closure();
    my_closure();
}

pub fn ex09() {
    // let my_string = String::from("I will go into the closure");
    // let my_vec = &vec![8, 9, 10];
    let my_vec = vec![8, 9, 10];
    let my_closure = || {
        // println!("{my_string}");
        my_vec.into_iter().for_each(|item| println!("{item}"));
    };
    my_closure();
    // my_closure();
}

pub fn ex10() {
    let my_string = String::from("Can I go inside the thread?");
    let handle = std::thread::spawn(move || {
        println!("{my_string}");
    });
    handle.join().unwrap();
}
pub fn ex11() {
    let mut join_handles = vec![];
    for num in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("Inside thread number: {num}");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
}

fn main() {
    ex11();
}

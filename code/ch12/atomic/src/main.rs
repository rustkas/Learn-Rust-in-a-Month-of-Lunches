pub fn ex01() {
    let handle = std::thread::spawn(|| println!("The thread is working!"));
    handle.join().unwrap();
    println!("Exiting the program");
}

pub fn ex02() {
    let handle = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("The thread is working!")
        }
    });
    handle.join().unwrap();
    println!("Exiting the program");
}
pub fn ex03() {
    let thread1 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 1 is working!")
        }
    });
    let thread2 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 2 is working!")
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Exiting the program");
}
pub fn ex04() {
    use std::sync::{Arc, Mutex};
    let my_number = Arc::new(Mutex::new(0));
    let cloned_1 = Arc::clone(&my_number);
    let cloned_2 = Arc::clone(&my_number);
    let thread1 = std::thread::spawn(move || {
        for _ in 0..10 {
            *cloned_1.lock().unwrap() += 1;
        }
    });
    let thread2 = std::thread::spawn(move || {
        for _ in 0..10 {
            *cloned_2.lock().unwrap() += 1;
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {my_number:?}");
    println!("Exiting the program");
}

pub fn ex05() {
    use std::sync::{Arc, Mutex};
    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];
    for _ in 0..10 {
        let my_number_clone = Arc::clone(&my_number);
        let handle = std::thread::spawn(move || {
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle);
    }
    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("{my_number:?}");
}
pub fn ex06() {
    use std::sync::{Arc, Mutex};
    use std::thread::spawn;

    fn make_arc(number: i32) -> Arc<Mutex<i32>> {
        Arc::new(Mutex::new(number))
    }
    fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
        Arc::clone(&input)
    }

    let mut handle_vec = vec![];
    let my_number = make_arc(0);
    for _ in 0..10 {
        let my_number_clone = new_clone(&my_number);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = my_number_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle);
    }
    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("{my_number:?}");
}
fn main() {
    ex06();
}

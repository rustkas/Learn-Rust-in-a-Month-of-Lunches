pub fn ex01() {
    use std::thread;

    thread::spawn(|| {});
    thread::spawn(|| {});
}

pub fn ex02() {
    use std::thread;
    thread::scope(|s| {
        s.spawn(|| {});
        s.spawn(|| {});
    });
}

pub fn ex03() {
    use std::sync::Mutex;
    use std::thread;

    let my_number = Mutex::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
    });
    println!("{}", my_number.lock().unwrap())
}

pub fn ex04() {
    use std::sync::Mutex;
    use std::thread;

    let mutex_number = Mutex::new(0);

    let mut regular_mut_number = 0;
    let regular_unmut_number = 0;

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..3 {
                *mutex_number.lock().unwrap() += 1;
                regular_mut_number += 1;
                println!(
                    "Multiple immutable borrows is fine!
{regular_unmut_number}"
                );
            }
        });
        s.spawn(|| {
            for _ in 0..3 {
                *mutex_number.lock().unwrap() += 1;
                // regular_mut_number += 1;
                println!("Borrowing {regular_unmut_number} here too, it's just fine!");
            }
        });
    });
    println!("mutex_number: {mutex_number:?}");
    println!("regular_mut_number: {regular_mut_number}");
}

fn main() {
    ex04();
}

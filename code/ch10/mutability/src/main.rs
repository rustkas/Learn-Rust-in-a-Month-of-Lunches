#[allow(dead_code)]
pub fn ex01() {
    struct PhoneModel {
        company_name: String,
        model_name: String,
        screen_size: f32,
        memory: usize,
        date_issued: u32,
        on_sale: bool,
    }

    impl PhoneModel {
        fn method_one(&self) {}
        fn method_two(&self) {}
    }

    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: true,
    };

    super_phone_3000.method_one();
    super_phone_3000.method_two();
}

// Cell
#[allow(dead_code)]
pub fn ex02() {
    use std::cell::Cell;

    #[derive(Debug)]
    struct PhoneModel {
        company_name: String,
        model_name: String,
        screen_size: f32,
        memory: usize,
        date_issued: u32,
        on_sale: Cell<bool>,
    }

    impl PhoneModel {
        fn method_one(&self) {}
        fn method_two(&self) {}
        fn make_not_on_sale(&self) {
            self.on_sale.set(false);
        }
    }

    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    super_phone_3000.method_one();
    super_phone_3000.method_two();

    super_phone_3000.make_not_on_sale();
    println!(
        "{name} = {on_sale}",
        name = stringify!(super_phone_3000.on_sale),
        on_sale = super_phone_3000.on_sale.get()
    );
    println!("{:?}", super_phone_3000.on_sale);
    println!("{super_phone_3000:#?}");
}

// RefCell
#[allow(dead_code)]
pub fn ex03() {
    use std::cell::RefCell;

    #[derive(Debug)]
    struct User {
        id: u32,
        year_registered: u32,
        username: String,
        active: RefCell<bool>,
    }

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{:?}", user_1.active);

    {
        let mut active = user_1.active.borrow_mut();
        *active = false;
    }

    println!("{user_1:#?}");
}

// RefCell
#[allow(dead_code)]
pub fn ex04() {
    use std::cell::RefCell;

    #[derive(Debug)]
    struct User {
        id: u32,
        year_registered: u32,
        username: String,
        active: RefCell<bool>,
    }

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{:?}", user_1.active);
    *user_1.active.borrow_mut() = false;

    println!("{user_1:#?}");
}

#[allow(dead_code)]
pub fn ex05() {
    use std::cell::RefCell;

    #[derive(Debug)]
    struct User {
        id: u32,
        year_registered: u32,
        username: String,
        active: RefCell<bool>,
    }

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    *user_1.active.borrow_mut() = !*user_1.active.borrow_mut();
    {
        let _borrow_one = user_1.active.borrow_mut();
    }
    let _borrow_one = user_1.active.try_borrow_mut();
    assert!(user_1.active.try_borrow().is_err());
    // let _borrow_two = user_1.active.try_borrow();
    // let _borrow_two = user_1.active.borrow_mut();
}

#[allow(dead_code)]
pub fn ex0501() {
    use std::cell::RefCell;

    #[derive(Debug)]
    struct User {
        id: u32,
        year_registered: u32,
        username: String,
        active: RefCell<bool>,
    }

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    {
        let mut value1 = user_1.active.borrow_mut();
        let value2 = *value1;
        *value1 = !value2;
    }

    let borrow_one = user_1.active.borrow_mut();
    drop(borrow_one);
    let borrow_one = user_1.active.borrow_mut();
    drop(borrow_one);
    let borrow_one = user_1.active.borrow_mut();
    drop(borrow_one);
    {
        let value1 = user_1.active.try_borrow_mut();
        if value1.is_ok() {
            let mut refmut = value1.unwrap();
            let mut boolean = *refmut;
            boolean = !boolean;
            *refmut = boolean;
        }
    }
    let _borrow_one = user_1.active.try_borrow_mut();
    assert!(user_1.active.try_borrow().is_err());
}

#[allow(unused_variables)]
pub fn ex06() {
    use std::cell::RefCell;

    let bool_in_refcell = RefCell::new(true);
    // std::thread::spawn(|| {
    // *bool_in_refcell.borrow_mut() = false;
    // });
}

pub fn ex07() {
    use std::sync::Mutex;

    let my_mutex = Mutex::new(5);
    println!("{name} = {my_mutex:#?}", name = stringify!(my_mutex));

    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        println!("{name} = {my_mutex:#?}", name = stringify!(my_mutex));
        println!(
            "{name} = {mutex_changer:?}",
            name = stringify!(mutex_changer)
        );

        *mutex_changer = 6;

        println!("{name} = {my_mutex:#?}", name = stringify!(my_mutex));
        println!(
            "{name} = {mutex_changer:?}",
            name = stringify!(mutex_changer)
        );
    }
    println!("{name} = {my_mutex:#?}", name = stringify!(my_mutex));
}

pub fn ex08() {
    use std::sync::Mutex;

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    *mutex_changer = 6;
    drop(mutex_changer);
    println!("{my_mutex:?}");
}

// dead lock example
#[allow(unused_variables, unused_mut)]
pub fn ex09() {
    use std::sync::Mutex;

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.lock().unwrap();
    println!("This will never print...");
}

pub fn ex0901() {
    use std::sync::Mutex;

    let my_mutex = Mutex::new(5);
    let _mutex_changer = my_mutex.lock().unwrap();
    let other_mutex_changer = my_mutex.try_lock();
    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {value}")
    } else {
        println!("Didn't get the lock")
    }
}

pub fn ex10() {
    use std::sync::Mutex;
    let my_mutex = Mutex::new(5);
    *my_mutex.lock().unwrap() = 6;
    println!("{name} = {my_mutex:#?}", name = stringify!(my_mutex));
}

pub fn ex11() {
    use std::sync::Mutex;

    let my_mutex = Mutex::new(5);
    for _ in 0..100 {
        *my_mutex.lock().unwrap() += 1;
    }
    println!("{name} = {my_mutex:#?}", name = stringify!(my_mutex));
}

pub fn ex12() {
    use std::sync::RwLock;
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("Values: {read1}, {read2}");
    let would_you_like_dead_lock = !false;
    if would_you_like_dead_lock {
        let _write1 = my_rwlock.write().unwrap();
    }
}

pub fn ex1201() {
    use std::sync::RwLock;
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");
    drop(read1);
    drop(read2);
    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    drop(write1);
    println!("{:?}", my_rwlock);
}

pub fn ex13() {
    use std::sync::RwLock;
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    drop(read1);
    println!("Content is  {read2:?}");
    drop(read2);
    if let Ok(mut number) = my_rwlock.try_write() {
        *number += 10;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };
}
fn main() {
    ex13();
}

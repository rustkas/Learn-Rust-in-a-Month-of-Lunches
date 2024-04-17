pub fn ex01() {
    fn takes_a_string(_unused_string: String) {}

    {
        let user_name = String::from("User MacUserson");
        takes_a_string(user_name);
        // println!("{user_name}");
    }

    {
        let user_name = String::from("User MacUserson");
        takes_a_string(user_name.clone());
        takes_a_string(user_name.clone());
    }
}

#[allow(dead_code)]
pub fn ex02() {
    #[derive(Debug)]
    struct City {
        name: String,
        population: u32,
        city_history: String,
    }
    #[derive(Debug)]
    struct CityData {
        names: Vec<String>,
        histories: Vec<String>,
    }

    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        city_history: "Calgary began as a fort called Fort Calgary that...".to_string(),
    };
    let canada_cities = CityData {
        names: vec![calgary.name],
        histories: vec![calgary.city_history],
    };
    //println!("Calgary's history is: {}", calgary.city_history);
    println!("{canada_cities:?}");
}

#[allow(dead_code)]
pub fn ex03() {
    use std::rc::Rc;
    #[derive(Debug)]
    struct City {
        name: Rc<String>,
        population: u32,
        city_history: Rc<String>,
    }
    #[derive(Debug)]
    struct CityData {
        names: Vec<Rc<String>>,
        histories: Vec<Rc<String>>,
    }

    let calgary_name = Rc::new("Calgary".to_string());
    let calgary_history =
        Rc::new("Calgary began as a fort called Fort Calgary that...".to_string());
    let calgary = City {
        name: Rc::clone(&calgary_name),
        population: 1_200_000,
        city_history: Rc::clone(&calgary_history),
    };
    let canada_cities = CityData {
        names: vec![Rc::clone(&calgary_name)],
        histories: vec![Rc::clone(&calgary_history)],
    };
    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));

    println!();
    println!(
        "Canada cities names are {city_names:?}",
        city_names = canada_cities.names
    );
    println!(
        "Canada city's histories are {histories:?}",
        histories = canada_cities.histories
    );
}

pub fn ex04() {
    use std::rc::Rc;

    fn takes_a_string(input: Rc<String>) -> Rc<String> {
        println!("It is: {input}");
        input
    }

    let user_name = Rc::new(String::from("User MacUserson"));
    let mut user_name_rc = Rc::clone(&user_name);
    user_name_rc = takes_a_string(user_name_rc);
    user_name_rc = takes_a_string(Rc::clone(&user_name_rc));

    println!("Strong count: {}", Rc::strong_count(&user_name_rc));
    println!("Weak count: {}", Rc::weak_count(&user_name_rc));
}

pub fn ex04_extra_01() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    struct User {
        name: String,
        friends: Vec<Weak<RefCell<User>>>,
    }

    impl User {
        fn new(name: String) -> Rc<RefCell<User>> {
            Rc::new(RefCell::new(User {
                name,
                friends: Vec::new(),
            }))
        }

        fn add_friend(user: Rc<RefCell<User>>, friend: Rc<RefCell<User>>) {
            user.borrow_mut().friends.push(Rc::downgrade(&friend));
            friend.borrow_mut().friends.push(Rc::downgrade(&user));
        }

        fn greet(&self) {
            println!("Hello, my name is {}!", self.name);
        }
    }

    let user1 = User::new(String::from("User1"));
    let user2 = User::new(String::from("User2"));

    User::add_friend(Rc::clone(&user1), Rc::clone(&user2));

    // Accessing a strong count
    println!("Strong count for user1: {}", Rc::strong_count(&user1));

    // Accessing a weak count
    let weak_user2 = Rc::downgrade(&user2);
    println!("Weak count for user2: {}", Weak::strong_count(&weak_user2));

    // Example usage of a weak pointer
    if let Some(user2) = weak_user2.upgrade() {
        user2.borrow().greet();
    } else {
        println!("User2 is no longer available!");
    }
}

pub fn ex04_extra_02() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    struct TreeNode {
        value: i32,
        parent: Weak<RefCell<TreeNode>>,
        children: Vec<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        fn new(value: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(TreeNode {
                value,
                parent: Weak::new(),
                children: Vec::new(),
            }))
        }

        fn add_child(parent: &Rc<RefCell<Self>>, child_value: i32) -> Rc<RefCell<Self>> {
            let child = Rc::new(RefCell::new(TreeNode {
                value: child_value,
                parent: Rc::downgrade(parent),
                children: Vec::new(),
            }));
            parent.borrow_mut().children.push(Rc::clone(&child));
            child
        }
    }

    let root = TreeNode::new(1);
    let child1 = TreeNode::add_child(&root, 2);
    let child2 = TreeNode::add_child(&root, 3);

    // Проверяем, есть ли доступ к родителю
    if let Some(parent) = child1.borrow().parent.upgrade() {
        println!("Child1's parent value: {}", parent.borrow().value);
    } else {
        println!("Child1 has no parent.");
    }

    if let Some(parent) = child2.borrow().parent.upgrade() {
        println!("Child2's parent value: {}", parent.borrow().value);
    } else {
        println!("Child2 has no parent.");
    }

    // Подсчет ссылок
    println!("Root strong count = {}", Rc::strong_count(&root)); // Выведет 1
    println!("Child1 strong count = {}", Rc::strong_count(&child1)); // Выведет 1
    println!("Child2 strong count = {}", Rc::strong_count(&child2)); // Выведет 1
    println!("Root weak count = {}", Rc::weak_count(&root)); // Выведет 2, потому что два ребенка имеют слабые ссылки на родителя
}

#[allow(dead_code)]
pub fn ex05() {
    #[derive(Debug)]
    struct City<'a> {
        name: &'a str,
        date_founded: u32,
    }
    #[derive(Debug)]
    struct Country<'a> {
        cities: Vec<City<'a>>,
    }
    #[derive(Debug)]
    struct World<'a> {
        countries: Vec<Country<'a>>,
    }

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

#[allow(dead_code)]
pub fn ex05_01() {
    use std::rc::Rc;
    #[derive(Debug)]
    struct City {
        name: Rc<String>,
        date_founded: u32,
    }
    #[derive(Debug)]
    struct Country {
        cities: Vec<City>,
    }
    #[derive(Debug)]
    struct World {
        countries: Vec<Country>,
    }
    impl World {}

    let city_names = vec![
        Rc::new("Ichinomiya".to_string()),
        Rc::new("Kurume".to_string()),
    ];
    let my_city = City {
        name: Rc::clone(&city_names[0]),
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

fn main() {
    ex05_01();
}

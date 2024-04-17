pub fn ex01() {
    fn do_something<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let some_vec = vec![9, 8, 10];
    let function = || {
        some_vec
            .into_iter()
            .for_each(|x| println!("The number is: {x}"));
    };
    do_something(function);
    // do_something(function);
}

pub fn ex02() {
    fn do_something<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let some_vec = vec![9, 8, 10];
    do_something(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
        println!();
    });
    do_something(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
    });
}

pub fn ex03() {
    fn takes_fn<F: Fn()>(f: F) {
        f();
        f();
    }
    let f = || print!("");
    takes_fn(f);
}

pub fn ex04() {
    fn takes_fnmut<F: FnMut()>(mut f: F) {
        f();
        f();
    }
    let mut index = 1;
    let mut string = String::new();
    let f = || {
        string.push_str("string");
        println!("{index}) {string}");
        println!();
        index += 1;
    };
    takes_fnmut(f);
    println!("After: {string}");
}

pub fn ex05() {
    fn takes_fnonce<F: FnOnce()>(f: F) {
        f();
    }
    fn takes_fnmut<F: FnMut()>(mut f: F) {
        f();
        f();
    }
    fn takes_fn<F: Fn()>(f: F) {
        f();
        f();
    }

    let mut my_string = String::from("Hello there");
    let prints_string = || {
        println!("{my_string}");
    };
    takes_fn(prints_string);

    println!();

    let adds_exclamation_and_prints = || {
        my_string.push('!');
        println!("{my_string}");
    };
    takes_fnmut(adds_exclamation_and_prints);

    println!();

    let prints_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    };
    takes_fnonce(prints_then_drops);
    // takes_fnonce(prints_then_drops);
}

pub fn ex06() {
    fn takes_fnonce<F: FnOnce()>(f: F) {
        f();
    }
    fn takes_fnmut<F: FnMut()>(mut f: F) {
        f();
        f();
    }
    fn takes_fn<F: Fn()>(f: F) {
        f();
        f();
    }

    let mut my_string = String::from("Hello there");
    takes_fn(|| {
        println!("{my_string}");
    });
    takes_fnmut(|| {
        my_string.push('!');
        println!("{my_string}");
    });
    takes_fnonce(|| {
        println!("Now dropping {my_string}");
        drop(my_string);
    });
}

pub fn ex07() {
    fn takes_fnonce<F: FnOnce()>(f: F) {
        f();
    }
    fn takes_fnmut<F: FnMut()>(mut f: F) {
        f();
        f();
    }

    let mut my_string = String::from("Hello there");
    let prints_string = || {
        println!("{my_string}");
    };
    takes_fnonce(prints_string);
    takes_fnmut(prints_string);
    let adds_exclamation_and_prints = || {
        my_string.push('!');
        println!("{my_string}");
    };
    takes_fnonce(adds_exclamation_and_prints);
    let prints_then_drops = || {
        println!("Now dropping \n\r {my_string}");
        drop(my_string);
    };
    takes_fnonce(prints_then_drops);
}

pub fn ex08() {
    fn takes_a_closure_and_does_nothing<F>(f: F)
    where
        F: Fn() -> i32,
    {
        f();
    }

    let my_closure = || 9;
    takes_a_closure_and_does_nothing(my_closure);
}

pub fn ex09() {
    fn takes_two_closures_and_does_nothing<F>(first: F, second: F)
    where
        F: Fn() -> i32,
    {
        first();
        second();
    }

    let first_closure = || 9;
    let second_closure = || 9;
    // takes_two_closures_and_does_nothing(first_closure, second_closure);
    takes_two_closures_and_does_nothing(first_closure, first_closure);
    takes_two_closures_and_does_nothing(second_closure, second_closure);
}

pub fn ex10() {
    fn takes_two_closures_and_does_nothing<F, G>(first: F, second: G)
    where
        F: Fn() -> i32,
        G: Fn() -> i32,
    {
        first();
        second();
    }

    let first_closure = || 9;
    let second_closure = || 9;
    takes_two_closures_and_does_nothing(first_closure, second_closure);
    takes_two_closures_and_does_nothing(first_closure, first_closure);
    takes_two_closures_and_does_nothing(second_closure, second_closure);
}
#[allow(dead_code)]
pub fn ex11() {
    #[derive(Debug)]
    struct City {
        name: String,
        years: Vec<u32>,
        populations: Vec<u32>,
    }

    impl City {
        fn change_city_data<F>(&mut self, mut f: F)
        where
            F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
        {
            f(&mut self.years, &mut self.populations)
        }
    }

    let mut tallinn = City {
        name: "Tallinn".to_string(),
        years: vec![1372, 1834, 1897, 1925, 1959, 1989, 2000, 2010, 2020],
        populations: vec![
            3_250, 15_300, 58_800, 119_800, 283_071, 478_974, 400_378, 406_703, 437_619,
        ],
    };

    tallinn.change_city_data(|x, y| {
        x.push(2030);
        y.push(500_000);
    });

    tallinn.change_city_data(|years, populations| {
        let new_vec = years
            .iter_mut()
            .zip(populations.iter_mut())
            .take(3)
            .collect::<Vec<(_, _)>>();
        println!("{new_vec:?}");
    });

    tallinn.change_city_data(|x, y| {
        let position_option = x.iter().position(|x| *x == 1834);
        if let Some(position) = position_option {
            println!(
                "Going to delete {} at position {:?} now.",
                x[position], position
            );
            x.remove(position);
            y.remove(position);
        }
    });
    println!(
        "Years left are {:?}
Populations left are {:?}",
        tallinn.years, tallinn.populations
    );
}

fn main() {
    ex11();
}

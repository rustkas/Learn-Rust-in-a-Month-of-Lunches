mod util;

pub fn ex01() {
    use std::collections::HashMap;

    struct City {
        name: String,
        population: HashMap<i32, i32>,
    }

    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    for (year, population) in tallinn.population {
        println!(
            "In {year}, {name} had a population of {population}.",
            name = tallinn.name
        );
    }
}

pub fn ex02() {
    use std::collections::BTreeMap;
    struct City {
        name: String,
        population: BTreeMap<i32, i32>,
    }

    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };
    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    for (year, population) in tallinn.population {
        println!(
            "In {year}, {name} had a population of {population}.",
            name = tallinn.name
        );
    }
}

pub fn ex03() {
    use std::collections::HashMap;

    let canadian_cities = ["Calgary", "Vancouver", "Gimli"];
    let german_cities = ["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();
    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }
    println!(
        "City = {city}. Country = {country:?}",
        city = "Bielefeld",
        country = city_hashmap["Bielefeld"]
    );
    println!(
        "City = {city}. Country = {country:?}",
        city = "Bielefeld",
        country = city_hashmap.get("Bielefeld").unwrap()
    );
    println!(
        "City = {city}. Country = {country:?}",
        city = "Bielefeldd",
        country = city_hashmap.get("Bielefeldd")
    );
}
pub fn ex04() {
    use std::collections::HashMap;

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    println!("Value is {:?}", book_hashmap.get(&1).unwrap());
}

pub fn ex05() {
    use std::collections::BTreeMap;
    let array = [
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "섀도우 오브 유어 스마일",
        "Eye of the World",
    ];
    let mut book_hashmap = BTreeMap::new();
    let mut key = 1;
    for item in array {
        match book_hashmap.get(&key) {
            Some(val) => println!("Key {key} has a value {val}"),
            None => {
                book_hashmap.insert(key, item);
                key += 1;
            }
        }
    }
    for (key, value) in book_hashmap {
        println!("key = {key}, value = {value}");
    }
}

pub fn ex06() {
    use std::collections::HashMap;

    let mut book_hashmap = HashMap::new();
    let mut old_hashmap_values = Vec::new();
    let hashmap_entries = [
        (1, "L'Allemagne Moderne"),
        (1, "Le Petit Prince"),
        (1, "섀도우 오브 유어 스마일"),
        (1, "Eye of the World"),
    ];
    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap.insert(key, value) {
            println!("Overwriting \"{old_value}\" with \"{value}\"!");
            old_hashmap_values.push(old_value);
        }
    }
    println!("All old values: {old_hashmap_values:?}");
}

pub fn ex07() {
    use std::collections::HashMap;
    let book_collection = [
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
        "Eye of the World",
    ];
    let mut book_hashmap = HashMap::new();
    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have \"{book}\"? - {true_or_false}");
    }
}

pub fn ex08() {
    use std::collections::HashMap;
    let book_collection = [
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
        "Eye of the World",
    ];
    let mut book_hashmap = HashMap::new();
    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        *return_value += 1;
    }
    for (book, number) in book_hashmap {
        println!("Do we have \"{book}\"? - {number}");
    }
}

pub fn ex09() {
    use std::collections::HashMap;
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];
    let mut survey_hash = HashMap::new();
    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }
    for (male_or_female, numbers) in survey_hash {
        println!("{male_or_female}: {numbers:?}");
    }
}

pub fn ex10() {
    use std::collections::BTreeSet;
    use std::collections::HashSet;

    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33, 50,
    ];

    println!("How many numbers in the Vec? {}", many_numbers.len());

    let mut number_hashset = HashSet::new();
    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    println!(
        "There are {hashset_length} unique numbers, so we are missing {}.",
        50 - hashset_length
    );
    println!("It does not contain: ");

    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            print!("{number} ");
        }
    }
    println!();
    println!("HashSet contains: ");
    let mut btree_set = BTreeSet::<u32>::new();
    for entry in number_hashset {
        print!("{} ", entry);
        btree_set.insert(entry as u32);
    }
    println!();
    println!("BTreeSet contains: ");
    for entry in btree_set {
        print!("{} ", entry);
    }
    println!();
}

pub fn ex11() {
    use std::collections::BTreeSet;

    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];
    let mut current_number = i32::MIN;
    let mut number_set = BTreeSet::new();
    for number in many_numbers {
        number_set.insert(number);
    }
    for number in number_set {
        if number < current_number {
            println!("This will never happen");
        }
        current_number = number;
        print!("{current_number} ");
    }
    println!();
}

pub fn ex12() {
    use std::collections::BinaryHeap;
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }
    println!("First item is largest, others are out of order: {heap:?}");
    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }
}

pub fn ex13() {
    use std::collections::BinaryHeap;

    let mut jobs = BinaryHeap::new();
    jobs.push((100, "Reply to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));
    for (_, job) in jobs {
        println!("You need to: {job}");
    }
}
pub fn ex14() {
    let mut my_vec = vec![9, 8, 7, 6, 5];
    my_vec.remove(0);
}
pub fn ex15() {
    let mut my_vec = vec![0; 600_000];
    for _ in 0..600000 {
        my_vec.remove(0);
    }
}
pub fn ex16() {
    use std::collections::VecDeque;
    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for _ in 0..600000 {
        my_vec.pop_front();
    }
}
fn main() {
    ex16();
}

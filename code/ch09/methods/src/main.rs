pub fn ex01() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect::<Vec<&str>>();
    println!("{:?}", filtered_months);
}
pub fn ex0101() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5 && month.contains("u"))
        .collect::<Vec<&str>>();
    println!("{:?}", filtered_months);
}

#[allow(dead_code)]
pub fn ex02() {
    struct Company {
        name: String,
        ceo: Option<String>,
    }
    impl Company {
        fn new(name: &str, ceo: &str) -> Self {
            let ceo = match ceo {
                "" => None,
                ceo => Some(ceo.to_string()),
            };
            Self {
                name: name.to_string(),
                ceo,
            }
        }
        fn get_ceo(&self) -> Option<String> {
            self.ceo.clone()
        }
    }

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let all_the_ceos = company_vec
        .iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();
    println!("{:?}", all_the_ceos);
}

pub fn ex03() {
    use std::num::ParseIntError;
    fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
        let parsed_number = match input.parse::<i32>() {
            Ok(number) => number,
            Err(e) => return Err(e),
        };
        println!("Number parsed successfully into {parsed_number}");
        Ok(parsed_number)
    }

    let ok_value = parse_and_log_str("5").ok();
    match ok_value {
        Some(result) => println!("Ok result = {result}"),
        None => println!("No result"),
    }
}

pub fn ex04() {
    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];
    let successful_numbers = user_input
        .iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
    println!("{:?}", successful_numbers);
}

#[allow(dead_code)]
pub fn ex05() {
    struct Company {
        name: String,
        ceo: Option<String>,
    }
    impl Company {
        fn new(name: &str, ceo: &str) -> Self {
            let ceo = match ceo {
                "" => None,
                ceo => Some(ceo.to_string()),
            };
            Self {
                name: name.to_string(),
                ceo,
            }
        }
        fn get_ceo(&self) -> Option<String> {
            self.ceo.clone()
        }
    }

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let results: Vec<Result<String, &str>> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or("No CEO found"))
        .collect();
    for item in results {
        println!("{:?}", item);
    }
}

#[allow(dead_code)]
pub fn ex06() {
    struct Company {
        name: String,
        ceo: Option<String>,
    }

    fn get_current_datetime() -> String {
        "2024-01-27T23:11:23".to_string()
    }

    impl Company {
        fn new(name: &str, ceo: &str) -> Self {
            let ceo = match ceo {
                "" => None,
                ceo => Some(ceo.to_string()),
            };
            Self {
                name: name.to_string(),
                ceo,
            }
        }
        fn get_ceo(&self) -> Option<String> {
            self.ceo.clone()
        }
    }

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let map_value = company_vec.iter().map(|company| {
        company.get_ceo().ok_or_else(|| {
            let err_message = format!("No CEO found for {}", company.name);
            println!("{err_message} at {}", get_current_datetime());
            err_message
        })
    });

    let results: Vec<Result<String, String>> = map_value.collect();

    results
        .iter()
        .filter(|res| res.is_ok())
        .for_each(|res| println!("{res:?}"));
}

pub fn ex07() {
    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];
    for index in 0..5 {
        char_vec.push(
            num_array
                .get(index)
                .and_then(|number| number.parse::<u32>().ok())
                .and_then(|number| char::try_from(number).ok()),
        );
    }
    println!("{:?}", char_vec);
}

pub fn ex0701() {
    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];
    for index in 0..5 {
        let char_option = num_array
            .get(index)
            .and_then(|number| number.parse::<u32>().ok())
            .and_then(|number| char::try_from(number).ok());
        if let Some(ch) = char_option {
            char_vec.push(ch);
        }
    }
    println!("{:?}", char_vec);
}

pub fn ex08() {
    let try_1 = [Some("Okay!"), None,           Some("Okay!"),  Some("Okay!"),  None];
    let try_2 = [None,          Some("Okay!"),  Some("Okay!"),  Some("Okay!"),  Some("Okay!"), ];
    let try_3 = [Some("Okay!"), Some("Okay!"),  Some("Okay!"),  Some("Okay!"),  None,];
    for i in 0..try_1.len() {
        println!("{:?}", try_1[i].and(try_2[i]).and(try_3[i]));
    }
}


fn main() {
    ex08();
}

#![allow(dead_code)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;


pub fn ex01() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}

pub fn ex02() {
    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)] // <-- this is a container attribute
    struct S {
        #[serde(default)] // <-- this is a field attribute
        f: i32,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "e")] // <-- this is also a container attribute
    enum E {
        #[serde(rename = "a")] // <-- this is a variant attribute
        A(String),
    }
}

// Функция для сериализации и десериализации, работающая с любыми типами, которые реализуют Serialize и Deserialize
pub fn serialize_deserialize<T>(value: &T)
where
    T: Serialize + DeserializeOwned + std::fmt::Debug, // DeserializeOwned требуется для корректной работы from_str
{
    // Сериализация значения
    let serialized = serde_json::to_string(value).unwrap();
    println!("serialized = {}", serialized);

    // Десериализация значения обратно в тип T
    let deserialized: T = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

pub fn ex03() {
    #[derive(Debug, Serialize, Deserialize)]
    struct W {
        a: i32,
        b: i32,
    }
    let w = W { a: 0, b: 0 }; // Represented as `{"a":0,"b":0}`

    // Convert the Point to a JSON string.
    serialize_deserialize(&w);

    #[derive(Debug, Serialize, Deserialize)]
    struct X(i32, i32);
    let x = X(0, 0); // Represented as `[0,0]`
    serialize_deserialize(&x);

    #[derive(Debug, Serialize, Deserialize)]
    struct Y(i32);
    let y = Y(0); // Represented as just the inner value `0`
    serialize_deserialize(&y);

    #[derive(Debug, Serialize, Deserialize)]
    struct Z;
    let z = Z; // Represented as `null`
    serialize_deserialize(&z);

    #[derive(Debug, Serialize, Deserialize)]
    enum E {
        W { a: i32, b: i32 },
        X(i32, i32),
        Y(i32),
        Z,
    }
    let w = E::W { a: 0, b: 0 }; // Represented as `{"W":{"a":0,"b":0}}`
    let x = E::X(0, 0); // Represented as `{"X":[0,0]}`
    let y = E::Y(0); // Represented as `{"Y":0}`
    let z = E::Z; // Represented as `"Z"`

    serialize_deserialize(&w);
    serialize_deserialize(&x);
    serialize_deserialize(&y);
    serialize_deserialize(&z);
}

// Default value for field
// https://riptutorial.com/rust/example/8980/default-value-for-field
pub fn ex04() {
    // #[macro_use] extern crate serde_derive;

    #[derive(Deserialize, Debug)]
    struct Request {
        // Use the result of a function as the default if "resource" is
        // not included in the input.
        #[serde(default = "default_resource")]
        resource: String,

        // Use the type's implementation of std::default::Default if
        // "timeout" is not included in the input.
        #[serde(default)]
        timeout: Timeout,

        // Use a method from the type as the default if "priority" is not
        // included in the input. This may also be a trait method.
        #[serde(default = "Priority::lowest")]
        priority: Priority,
    }

    fn default_resource() -> String {
        "/".to_string()
    }

    /// Timeout in seconds.
    #[derive(Deserialize, Debug)]
    struct Timeout(u32);
    impl Default for Timeout {
        fn default() -> Self {
            Timeout(30)
        }
    }

    #[derive(Deserialize, Debug)]
    enum Priority {
        ExtraHigh,
        High,
        Normal,
        Low,
        ExtraLow,
    }
    impl Priority {
        fn lowest() -> Self {
            Priority::ExtraLow
        }
    }

    let json = r#"
[
  {
    "resource": "/users"
  },
  {
    "timeout": 5,
    "priority": "High"
  }
]
"#;

    let requests: Vec<Request> = serde_json::from_str(json).unwrap();

    // The first request has resource="/users", timeout=30, priority=ExtraLow
    println!("{:?}", requests[0]);

    // The second request has resource="/", timeout=5, priority=High
    println!("{:?}", requests[1]);
}

pub fn ex05() {
    
}

fn main() {
    ex05();
}

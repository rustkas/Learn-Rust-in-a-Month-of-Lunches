pub fn ex01() {
    let client = reqwest::blocking::Client::default();
    let _ = client.get("https://www.rust-lang.org").send();
}

pub fn ex02() {
    let client = reqwest::blocking::Client::default();
    let response = client.get("https://www.rust-lang.org").send().unwrap();

    println!("{}", response.text().unwrap());
}

pub async fn ex03() {
    let client = reqwest::Client::default();
    let response = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap());
}
#[tokio::main]
async fn main() {
    let _ = ex03();
}

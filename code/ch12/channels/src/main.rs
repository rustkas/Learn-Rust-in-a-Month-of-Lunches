#[allow(unused_variables)]
pub fn ex01() {
    use std::sync::mpsc::{channel, Receiver, Sender};

    // let (sender, receiver) = channel();
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();
}
#[allow(unused_must_use)]
pub fn ex02() {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();
    sender.send(5);
    receiver.recv();
}

pub fn ex03() {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();

    sender.send(5).unwrap();
    println!("{}", receiver.recv().unwrap());
}

pub fn ex04() {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    std::thread::spawn(move || {
        sender.send("Send a &str this time").unwrap();
        sender.send("Send a &str this time").unwrap();
    });
    std::thread::spawn(move || {
        sender_clone.send("And here is another &str").unwrap();
        sender_clone.send("And here is another &str").unwrap();
    });
    while let Ok(res) = receiver.recv() {
        println!("{res}");
    }
}

pub fn ex0401() {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    std::thread::spawn(move || {
        sender.send("Send a &str this time").unwrap();
        sender.send("Send a &str this time").unwrap();
    });
    std::thread::spawn(move || {
        sender_clone.send("And here is another &str").unwrap();
        sender_clone.send("And here is another &str").unwrap();
    });
    while let Ok(res) = receiver.try_recv() {
        println!("{res}");
    }
}

pub fn ex05() {
    use std::sync::mpsc::channel;
    let (sender, receiver) = channel();
    drop(receiver);
    if let Err(e) = sender.send(5) {
        println!("Got an error: {e}")
    }
}

pub fn ex06() {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();
    sender.send(5).unwrap();
    sender.send(5).unwrap();
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
}

pub fn ex07() {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();
    sender.send(5).unwrap();
    sender.send(5).unwrap();
    drop(sender);

    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
}
fn main() {
    ex07();
}

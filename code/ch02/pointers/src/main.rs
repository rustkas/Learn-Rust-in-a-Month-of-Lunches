#[allow(unused_variables)]
fn main() {
    let my_number = 15;
    let single_reference = &my_number;
    let double_reference = &single_reference;
    let five_references = &&&&&my_number;
}

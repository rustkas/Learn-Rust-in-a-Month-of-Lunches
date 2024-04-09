fn main() {
    let my_number;
    {
        let calculation_result = { 57 };
        my_number = calculation_result;
        println!("info. {my_number}");
    }
    println!("out. {my_number}");
}

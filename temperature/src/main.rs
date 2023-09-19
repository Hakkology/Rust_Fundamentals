fn main() {
    
    println!("Please enter your temperature along with its unit (F for Fahrenheit, C for Celsius)");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

}

fn main() {

    print_hello();
    print_number_request(25);
    print_labeled_measurement(4, 'm');
    print_goodbye();
}

fn print_hello(){
    println!("Hello, world!");
}

fn print_goodbye() {
    println!("Goodbye world!.");
}

fn print_number_request(x: i32) {
    println!("The value of x is: {double_the_number(x)}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn double_the_number(x: i32) -> i32 {
    x = x * 2;
}

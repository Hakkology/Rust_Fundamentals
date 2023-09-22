fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // There can only be one mutable reference, two would fail.

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // data race conditions:
    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data.

    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
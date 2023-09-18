use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let _spaces = "   ";
    let _spaces = _spaces.len();
    // here spaces is once again immutable through let. This allows us to change the type of variable.

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // this context wont work because type is already given as string.
    // The other difference between mut and shadowing is that because we’re effectively creating a new variable 
    // when we use the let keyword again, we can change the type of the value but reuse the same name.

    let _guess: u32 = "42".parse().expect("Not a number!");

    // i8-i128 represents signed integers / u8-u128 represents unsigned integers <similar to uint32_t>

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of b is: {b}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let array = [1, 2, 3, 4, 5];

    // Tuples: Can contain elements of different types. In your example, tup has an integer, a float, and another integer.
    // Arrays: All elements of an array must be of the same type.
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type annotation for an array and the qty of elements inside the array.
    let first = a[0]; // as usual, arrays are accessed by indexing


}

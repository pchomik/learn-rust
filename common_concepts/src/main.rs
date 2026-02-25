// Based on https://www.youtube.com/watch?v=2V0JaMVjzws&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=3

fn variables() {
    // Variables and mutability
    let mut x = 5;
    println!("The value of x variable is {}", x);
    x = 10;
    println!("The value of x variable is {}", x);
}

fn constant() {
    // Constant variable
    const SUBSCRIBER_LIMIT: u32 = 100_000;
}

fn shadowing() {
    //Shadowing
    let y: i32 = 5;
    println!("The value of y variable is {}", y);
    let y: &str = "test";
    println!("The value of y variable is {}", y);
}

fn single_value_data_types() {
    // DATA TYPES
    // Integers
    //    - 8-bit     i8      u8
    //    - 16-bit    i16     u16
    //    - 32-bit    i32     u32
    //    - 64-bit    i64     u64
    //    - 128-bit   i128    u128
    //    - arch      isize   usize

    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    // Overflow
    let f: u8 = 255;

    // Floating-point numbers
    let ff = 2.0;
    let g = 3.0;

    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 10 - 5;

    // multiplication
    let multi = 4 * 30;

    // division
    let div = 56.7 / 32.2;

    // remainder
    let rem = 43 % 5;

    // Booleans
    let t = true;
    let x = false;

    // Character
    let c = 'z';
    let z = 'Z';
}

fn compand_types() {}

fn main() {
    variables();
    constant();
    shadowing();
    single_value_data_types();
    compand_types();
}

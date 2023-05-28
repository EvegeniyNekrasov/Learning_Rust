/*
    Ownership rules:
    - Each value in Rust has an Owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped
*/

pub fn clone_data() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // we can do this because the int has a known size at compile time is stored entirely on the stack
    // so copies of the actual values are quick to make
    //  In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.
    let x: i32 = 5;
    let y: i32 = x;
    println!("x = {}, y = {}", x, y);
}

pub fn ownership_and_function() {
    let s = String::from("Hello");
    take_ownership(s);

    let x = 5;
    make_copy(x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
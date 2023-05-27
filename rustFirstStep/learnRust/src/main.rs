fn main() {
    // Variables and Mutability
    let mut x = 5; // to able to assign another value in the future the variable must be mutable
    // println!("The value of x is: {x}");
    x = 6;
    // println!("The value of x is: {x}");

    // Constant
    /*
        Rust's name conversations for constants is to use all
        uppercase with underscores between words.
        Constants are valid for the entire time a program runs, within the scope in which they were declared.
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("The values if THREE_HOURS_IN_SECONDS id: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1; // 6

    {
        let x = x * 2; //12
    }

    let spaces = "   "; // string
    let spaces = spaces.len(); // number

    // Compound types
    /*
        Rust has two primitive compound types: tuples and arrays
    */

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //destructuring tuple
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;

    // println!("The value of x is: {x}"); // output: The value of x is: 500
    // println!("The value of y is: {y}"); // output: The value of x is: 6.4
    // println!("The value of z is: {z}"); // output: The value of x is: 1

    // access tuple index
    let some_tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = some_tup.0;
    let six_point_four = some_tup.1;
    let one = some_tup.2;

    // An array type
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    another_function(23);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
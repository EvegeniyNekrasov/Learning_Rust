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
}

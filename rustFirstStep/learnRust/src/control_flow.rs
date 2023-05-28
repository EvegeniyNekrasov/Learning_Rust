pub fn is_three(x: i32) {
    /*
        Using to many 'else if' expressions can clutter your code, so if you have more than one,
        you might want to refactor your code.
    */
    if x != 3 {
        println!("It's not a number tree");
    } else {
        println!("The number is tree!");
    }
}

pub fn let_if_in_let_expression() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

pub fn values_from_loop() {
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // you can add the value you want returned after the break expression you use to stop the loop
        }
    };

    println!("The result is {result}");
}

pub fn labeled_loop() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 { break; } // this break will only exit the inner loop
            if count == 2 { break 'counting_up; } // this break with 'counting_up label will exit the outer loop
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

pub fn loop_with_while() {
    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFFF!!!!");
}

pub fn loop_with_for() {
    let a = [10, 20, 30, 40, 50, 60];

    for element in a {
        println!("The value is: {element}");
    }
}
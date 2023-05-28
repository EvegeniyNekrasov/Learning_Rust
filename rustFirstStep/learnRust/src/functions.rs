pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

pub fn y_value() {
    let y = {
        let x = 3;
        x + 1 // this is an expression and don't need a semicolon at the and
    };

    println!("The value of y is: {:?}", y)
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub fn struct_user() {
    let mut user1 = build_user(String::from("someemail@example.com"), String::from("someuser123"));

    let user2 = User {
        email: String::from("somenewmail@example.com"),
        ..user1 // must come last to specify that any remaining fields should get their values from the corresponding fields in user1
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
    println!("{}", user2.username);
    println!("{}", user2.email);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn tuple_struct() {
    let black = Color(255,255,123);
    let origin = Point(0,0,0);
    println!("{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
}

// Using struct
#[derive(Debug)] // so we can print debugging information
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn calculate_area_of_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height // immutable borrow of struct Rectangle instance
}


// Method Syntax

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 { // borrows the 'self' instance
        self.width * self.height
    }
}

pub fn calculate_area_of_rectangle2() {
    let rect2 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!("rect2 id {:?}", rect2);
}
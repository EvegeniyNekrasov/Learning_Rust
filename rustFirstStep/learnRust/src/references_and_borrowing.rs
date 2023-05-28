pub fn reference_to_an_object() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // goes out of scope, but because it does not have ownership, it's not dropped

pub fn mutable_references() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

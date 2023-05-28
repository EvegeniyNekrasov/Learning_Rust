/*
    Slice let you reference a contiguous sequence of elements in a collection rather that the whole
    collection. A slice is a kind of reference, so it does not have ownership
*/

pub fn slicing() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error!
    // println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


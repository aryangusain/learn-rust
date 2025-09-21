fn main() {
    let str = String::from("hello aryan");
    let res = find_first_a(&str);
    match res {
        None => print!("a is not present in string"),
        Some(idx) => print!("a is present at index: {}", idx),
    }
}

fn find_first_a(str: &String) -> Option<usize> {
    for (idx, character) in str.chars().enumerate() {
        if character == 'a' {
            return Some(idx);
        }
    }

    return None;
}

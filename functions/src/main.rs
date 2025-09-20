fn main() {
    let x: i32 = 100;
    let result: String = if is_even(x) {String::from("even")} else {String::from("odd")};
    println!("x is {}", result);

    println!("{}", greet());
}

fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}

fn greet() -> String {
    return String::from("Hi there!");
}
fn main() {
    let x: i32 = 5;
    let y: i32 = 10;

    if is_even(x) {
        println!("x is even");
    }
    else {
        println!("x is odd");
    }

    if is_even(y) {
        println!("y is even");
    }
    else {
        println!("y is odd");
    }
}

fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}
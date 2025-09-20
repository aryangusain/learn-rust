fn main() {
    let is_male: bool = true;
    let is_above_18: bool = false;

    if is_male {
        println!("you are a male!");
    }
    else {
        println!("you are a female");
    }

    if is_male && is_above_18 {
        println!("have a beer!");
    }
    else {
        println!("you are not allowed here!");
    }
}

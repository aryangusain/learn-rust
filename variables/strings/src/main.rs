fn find_first_name(name: &String) -> &str {
    let mut index = 0;

    for (_, c) in name.chars().enumerate() {
        if c == ' ' {
            break;
        }
        else {
            index += 1;
        }
    }

    return &name[0..index];
}

fn main() {
    // let mut name: String = String::from("Aryan");
    // name.push_str(" Gusain");
    // println!("Full name: {}", name);
    // name.replace_range(5..name.len(), "");
    // println!("First name: {}", name);

    // let name = String::from("Aryan Gusain");
    // let first_name = &name[0..5];
    // let last_name = &name[6..name.len()];
    // println!("First Name: {}, Last Name: {}", first_name, last_name);

    let name = String::from("Aryan Gusain");
    let first_name = find_first_name(&name);
    println!("First Name: {}", first_name);
}

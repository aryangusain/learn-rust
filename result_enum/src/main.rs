use std::fs;

fn main() {
    let res = fs::read_to_string("./abc.txt");

    match res {
        Ok(content) => {
            println!("File Content: {}", content);
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

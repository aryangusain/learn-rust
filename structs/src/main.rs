struct User {
    active: bool,
    username: String,
    email: String,
    age: u32,
}

impl User {
    fn display_data(&self) {
        println!("user active: {}", self.active);
        print!("username: {}, email: {}, age: {}", self.username, self.email, self.age);
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("'aryangusain_'"),
        email: String::from("'aryangusain123@gmail.com'"),
        age: 21,
    };
    
    user1.display_data();
}

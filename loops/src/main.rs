fn main() {
    for i in 0..10 {
        print!("{}, ", i);
    }
    println!();

    let sentence = String::from("hello world from India");
    let first_word = get_first_word(sentence);
    println!("first word of sentence is : {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::new();

    for ch in sentence.chars() {
        ans.push(ch);
        if ch == ' ' {
            break;
        }
    }

    return ans;
}

use std::collections::HashMap;

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for (key, value) in vec {
        map.insert(key, value);
    }
    return map;
}

fn main() {
    let input_vec = vec![(String::from("aryan"), 22), (String::from("raman"), 26), (String::from("anurag"), 30)];
    let hashmap = group_values_by_keys(input_vec);
    println!("{:?}", hashmap);
}

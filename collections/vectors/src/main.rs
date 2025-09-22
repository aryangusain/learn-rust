fn filter_vector(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        }
        else {
            i += 1;
        }
    }
}

fn main() {
    let mut vec = Vec::new();
    for i in 0..11 {
        vec.push(i);
    }
    println!("original vector: {:?}", vec);
    filter_vector(&mut vec);
    println!("vector with even values: {:?}", vec);
}
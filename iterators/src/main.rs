/*
    iter()                  iter_mut()              into_iter()
    - immutable reference   - mutable reference     - ownership

    Adapters
    - Consumer adapters: consume the iterator  (eg. sum())
    - Iterator adapters: don't consume the iterator (eg. map(), filter())
*/

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let v_iter = vec.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);

    let ans: Vec<i32> = v_iter.collect();   

    println!("{:?}", ans);
}

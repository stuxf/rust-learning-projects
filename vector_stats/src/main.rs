use std::collections::HashMap;
fn main() {
    let mut input_vec = vec![2, 4, 1, 23, 1, 5, 2, 6];
    input_vec.sort();
    dbg!(&input_vec);
    if input_vec.len() % 2 == 0 {
        let mid = input_vec.len() / 2;
        let sum = input_vec[mid - 1] + input_vec[mid];
        println!("The median is: {}", sum / 2);
    } else {
        let mid = input_vec.len() / 2;
        println!("The median is: {}", input_vec[mid]);
    }
    let mut mode_map = HashMap::new();
    for i in input_vec {
        let count = mode_map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut mode = 0;
    for (key, value) in mode_map {
        if value > max_count {
            max_count = value;
            mode = key;
        }
    }
    println!("The mode is: {}", mode);
}

use std::collections::HashMap;

fn main() {
    let vec1 = vec![1, 6, 1, 2, 9, 1, 3, 7, 8, 6, 1, 4];
    let vec2 = vec![8, 5, 6, 5, 7, 4, 2, 1, 5, 3, 9, 5];

    println!(
        "vec1, mean: {}, median: {}, mode: {}",
        find_mean(&vec1),
        find_median(&vec1),
        find_mode(&vec1)
    );
    println!(
        "vec2, mean: {}, median: {}, mode: {}",
        find_mean(&vec2),
        find_median(&vec2),
        find_mode(&vec2)
    );
}

fn find_mean(vec: &Vec<i32>) -> f64 {
    let mut total = 0;
    for n in vec {
        total += n;
    }

    total as f64 / vec.len() as f64
}

fn find_median(vec: &Vec<i32>) -> f64 {
    let mut vec_clone = vec.clone();
    vec_clone.sort();

    let len = vec_clone.len() as f64;
    let middle_index: f64 = len / 2.0;

    if len % 2.0 == 0.0 {
        (&vec_clone[(middle_index - 1.0) as usize] + &vec_clone[middle_index as usize]) as f64 / 2.0
    } else {
        *&vec_clone[middle_index as usize].into()
    }
}

fn find_mode(vec: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let result: (i32, i32) = (0, 0);
    let (mut r_key, mut r_value) = result;

    for n in vec {
        let count = map.entry(*n).or_insert(0);
        *count += 1;
    }

    for (key, value) in map {
        if r_value < value {
            r_key = key;
            r_value = value;
        }
    }

    r_key
}

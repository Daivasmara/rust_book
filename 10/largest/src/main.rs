fn main() {
    let number_list1 = vec![34, 50, 25, 100, 65];
    let result1 = largest(&number_list1);
    println!("Largest 1st: {}", result1);
    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result2 = largest(&number_list2);
    println!("Largest 2nd: {}", result2);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

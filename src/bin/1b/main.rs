use std::collections::HashMap;

use advent_of_code_2024::helpers::file::read_two_number_columns_input;

fn main() {
    let mut list1: [i32; 1000] = [0; 1000];
    let mut list2: [i32; 1000] = [0; 1000];

    read_two_number_columns_input(&mut list1, &mut list2);

    let mut map: HashMap<i32, i32> = HashMap::new();

    for item in list1 {
        match map.get(&item) {
            Some(_) => continue,
            None => {
                map.insert(item, 0);
            }
        }
    }

    for item in list2 {
        match map.get(&item) {
            None => continue,
            Some(value) => {
                map.insert(item, value + 1);
            }
        }
    }

    let mut sum = 0;

    for (k, v) in map.iter() {
        sum += k * v;
    }

    println!("{}", sum);
}

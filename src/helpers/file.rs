use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_two_number_columns_input(list1: &mut [i32; 1000], list2: &mut [i32; 1000]) {
    let file = File::open("./input1.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut line_index = 0;
    for line in lines.flatten() {
        let numbers: Vec<&str> = line.trim().split("   ").collect();

        list1[line_index] = numbers[0].parse().unwrap();
        list2[line_index] = numbers[1].parse().unwrap();

        line_index += 1;
    }
}

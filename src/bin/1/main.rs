use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input(list1: &mut [i32; 1000], list2: &mut [i32; 1000]) {
    let file = File::open("./src/bin/1/input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut line_index = 0;
    for line in lines.flatten() {
        let numbers: Vec<&str> = line.trim().split("   ").collect();

        list1[line_index] = numbers[0].parse().unwrap();
        list2[line_index] = numbers[1].parse().unwrap();

        line_index += 1;
    }
}

fn main() {
    let mut list1: [i32; 1000] = [0; 1000];
    let mut list2: [i32; 1000] = [0; 1000];

    read_input(&mut list1, &mut list2);

    list1.sort_unstable();
    list2.sort_unstable();

    let mut total_dist = 0;
    for i in 0..1000 {
        let distance = (list1[i] - list2[i]).abs();
        total_dist += distance;
    }

    println!("Total distance is {}", total_dist);
}

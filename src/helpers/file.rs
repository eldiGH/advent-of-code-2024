use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Read},
    iter::Flatten,
};

pub fn read_lines(path: &str) -> Flatten<Lines<BufReader<File>>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().flatten()
}

pub fn read_row_of_numbers(path: &str) -> impl Iterator<Item = Vec<i32>> {
    read_lines(path).map(|line| {
        let mut vec: Vec<i32> = vec![];

        let entries = line.split(" ");

        for entry in entries {
            vec.push(entry.parse().unwrap());
        }

        vec
    })
}

pub fn read_two_number_columns_input(list1: &mut [i32; 1000], list2: &mut [i32; 1000]) {
    let mut line_index = 0;
    for line in read_lines("./inputs/1.txt") {
        let numbers: Vec<&str> = line.trim().split("   ").collect();

        list1[line_index] = numbers[0].parse().unwrap();
        list2[line_index] = numbers[1].parse().unwrap();

        line_index += 1;
    }
}

pub fn read_whole(path: &str) -> String {
    let mut file = File::open(path).unwrap();

    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    return buf;
}

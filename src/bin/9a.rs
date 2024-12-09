use std::{fs::File, io::Write};

use advent_of_code_2024::helpers::file::read_whole;

const EMPTY_SPACE_VALUE: u16 = std::u16::MAX;

fn main() {
    let disk = read_whole("./inputs/3.txt");

    let mut blocks: Vec<u16> = vec![];

    let mut is_empty = false;
    let mut id = 0;
    let mut position = 0;

    let mut first_empty_space_index = std::usize::MAX;

    for block in disk.chars() {
        let count = block.to_digit(10).unwrap();

        let value = if is_empty { EMPTY_SPACE_VALUE } else { id };

        if first_empty_space_index == std::usize::MAX && is_empty {
            first_empty_space_index = position;
        }

        for _ in 0..count {
            blocks.push(value);
            position += 1;
        }

        if !is_empty {
            id += 1;
        }
        is_empty = !is_empty;
    }

    for i in (0..blocks.len()).rev() {
        if i <= first_empty_space_index {
            break;
        }

        if blocks[i] == EMPTY_SPACE_VALUE {
            continue;
        }

        blocks.swap(first_empty_space_index, i);

        first_empty_space_index += 1;

        if first_empty_space_index >= i - 1 {
            break;
        }

        while blocks[first_empty_space_index] != EMPTY_SPACE_VALUE {
            first_empty_space_index += 1;
        }
    }

    let mut sum: u64 = 0;

    for (i, id) in blocks.iter().enumerate() {
        if *id == EMPTY_SPACE_VALUE {
            break;
        }

        sum += (i as u64) * ((*id) as u64);
    }

    let mut file = File::create("./test.txt").unwrap();

    for (i, id) in blocks.iter().enumerate() {
        file.write_fmt(format_args!("{}. {}\n", i, id)).unwrap();
    }

    println!("{}", sum);
}

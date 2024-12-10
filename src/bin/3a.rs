use advent_of_code_2024::helpers::file::read_whole;
use regex::Regex;

fn main() {
    let data = read_whole("./inputs/3.txt");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;

    for result in re.captures_iter(&data) {
        let x: i32 = result[1].parse().unwrap();
        let y: i32 = result[2].parse().unwrap();

        sum += x * y;
    }

    println!("{}", sum);
}

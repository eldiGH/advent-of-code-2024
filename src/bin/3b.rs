use advent_of_code_2024::helpers::file::read_whole;
use regex::Regex;

fn main() {
    let data = read_whole("./inputs/3.txt");

    let mut disabled_boundaries: Vec<(usize, usize)> = vec![];

    let mut boundary_index = 0;

    loop {
        let don_t_index = match data[boundary_index..].find("don't()") {
            Some(i) => i + boundary_index,
            None => break,
        };

        let do_index = match data[don_t_index..].find("do()") {
            Some(i) => i + don_t_index,
            None => data.len(),
        };

        disabled_boundaries.push((don_t_index, do_index));

        boundary_index = do_index;
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;

    'outer: for result in re.captures_iter(&data) {
        let location = result.get(0).unwrap().start();

        for (start, end) in &disabled_boundaries {
            if location > *start && location < *end {
                continue 'outer;
            }
        }

        let x: i32 = result[1].parse().unwrap();
        let y: i32 = result[2].parse().unwrap();

        sum += x * y;
    }

    println!("{}", sum);
}

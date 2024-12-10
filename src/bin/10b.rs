use std::vec;

use advent_of_code_2024::helpers::file::read_lines;

struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn check_path(point: Point, map: &Vec<Vec<usize>>, map_width: usize, map_height: usize) -> u32 {
    let value = map[point.y][point.x];

    if value == 9 {
        return 1;
    }

    let mut score: u32 = 0;

    for (relative_x, relative_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        if (point.y == 0 && relative_y == -1)
            || (point.y == map_height - 1 && relative_y == 1)
            || (point.x == 0 && relative_x == -1)
            || (point.x == map_width - 1 && relative_x == 1)
        {
            continue;
        }

        let y = ((point.y) as i32 + relative_y) as usize;
        let x = ((point.x as i32) + relative_x) as usize;

        let next_value = map[y][x];

        if next_value == value + 1 {
            score += check_path(Point { x, y }, map, map_width, map_height);
        }
    }

    score
}

fn main() {
    let map: Vec<Vec<usize>> = read_lines("./inputs/10.txt")
        .map(|row| {
            row.chars()
                .map(|height| height.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let map_width = map.len();
    let map_height = map[0].len();

    let mut scores: Vec<u32> = vec![];

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height != 0 {
                continue;
            }

            let score = check_path(Point { x, y }, &map, map_width, map_height);

            if score > 0 {
                scores.push(score);
            }
        }
    }

    let total_score: u32 = scores.iter().sum();

    println!("{}", total_score);
}

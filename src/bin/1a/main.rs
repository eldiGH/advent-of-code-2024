use advent_of_code_2024::helpers::file::read_two_number_columns_input;

fn main() {
    let mut list1: [i32; 1000] = [0; 1000];
    let mut list2: [i32; 1000] = [0; 1000];

    read_two_number_columns_input(&mut list1, &mut list2);

    list1.sort_unstable();
    list2.sort_unstable();

    let mut total_dist = 0;
    for i in 0..1000 {
        let distance = (list1[i] - list2[i]).abs();
        total_dist += distance;
    }

    println!("Total distance is {}", total_dist);
}

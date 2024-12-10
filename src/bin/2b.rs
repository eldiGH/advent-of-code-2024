use advent_of_code_2024::helpers::file::read_row_of_numbers;

fn copy_vec_without_index(vec: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut copied_vec: Vec<i32> = vec![];
    for (i, element) in vec.iter().enumerate() {
        if i != index {
            copied_vec.push(*element);
        }
    }

    copied_vec
}

fn is_report_safe(report: &Vec<i32>, dampened: bool) -> bool {
    let is_increasing = report[0] < report[1];

    let mut i = 1;

    while i < report.len() {
        let alert = report[i];
        let previous_alert = report[i - 1];

        let difference = (alert - previous_alert).abs();

        if (previous_alert < alert) != is_increasing
            || (difference < 1 || difference > 3)
            || (is_increasing && previous_alert > alert)
            || (!is_increasing && previous_alert < alert)
        {
            if dampened {
                return false;
            }

            for i in 0..report.len() {
                if is_report_safe(&copy_vec_without_index(report, i), true) {
                    return true;
                }
            }

            println!("{:?}", report);
            return false;
        }

        i += 1;
    }

    true
}

fn main() {
    let mut safe_reports = 0;

    for report in read_row_of_numbers("./inputs/2.txt") {
        if is_report_safe(&report, false) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports)
}

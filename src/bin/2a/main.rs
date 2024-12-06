use advent_of_code_2024::helpers::file::read_row_of_numbers;

fn is_report_safe(report: &Vec<i32>) -> bool {
    let is_increasing = report[0] < report[1];

    for (previous_alert, alert) in report
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, alert)| (&report[index - 1], alert))
    {
        let difference = (alert - previous_alert).abs();

        if difference < 1 || difference > 3 {
            return false;
        }

        if (is_increasing && previous_alert > alert) || (!is_increasing && previous_alert < alert) {
            return false;
        }
    }

    true
}

fn main() {
    let mut safe_reports = 0;

    for report in read_row_of_numbers("./inputs/2.txt") {
        if is_report_safe(&report) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports)
}

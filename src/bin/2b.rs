use advent_of_code_2024::helpers::file::read_row_of_numbers;

fn is_report_safe(report: &Vec<i32>, dampened: bool) -> bool {
    let is_increasing = report[0] < report[1];

    let mut i = 1;

    println!("{:?}", report);

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

            let mut new_report = report
                .clone()
                .into_iter()
                .take(i)
                .skip(1)
                .take(999)
                .collect();

            if is_report_safe(&new_report, true) {
                return true;
            }

            new_report = report
                .clone()
                .into_iter()
                .take(i - 1)
                .skip(1)
                .take(999)
                .collect();
            return is_report_safe(&new_report, true);
        }

        i += 1;
    }

    true
}

fn main() {
    let safe_reports: i32 = read_row_of_numbers("./inputs/2.txt")
        .map(|report| is_report_safe(&report, false) as i32)
        .sum();

    println!("Safe reports: {}", safe_reports)
}

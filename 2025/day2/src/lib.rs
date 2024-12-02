pub fn solve_task_one(file: &str) -> usize {
    let mut reports = vec![];
    for line in file.trim().lines() {
        reports.push(
            line.split_ascii_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }
    reports
        .iter()
        .filter(|report| is_safe_level(report))
        .count()
}

fn is_safe_level(report: &[usize]) -> bool {
    let valid_ordering = report[0].cmp(&report[1]);
    for i in 0..report.len() - 1 {
        let first = report[i];
        let second = report[i + 1];

        if first == second {
            return false;
        }

        let diff = first.abs_diff(second);
        if !(1..=3).contains(&diff) {
            return false;
        }
        if first.cmp(&second) != valid_ordering {
            return false;
        }
    }
    true
}

pub fn solve_task_two(file: &str) -> usize {
    let mut reports = vec![];
    for line in file.trim().lines() {
        reports.push(
            line.split_ascii_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }
    reports
        .iter()
        .filter(|report| is_safe_level_2(report, None))
        .count()
}

pub fn is_safe_level_2(report: &[usize], skip: Option<usize>) -> bool {
    let valid_ordering = report[0].cmp(&report[1]);

    for i in 0..report.len() - 1 {
        let first = report[i];
        let second = report[i + 1];

        if first == second {
            if skip.is_none() {
                let mut first_removed = report.to_vec();
                first_removed.remove(i);
                let mut second_removed = report.to_vec();
                second_removed.remove(i+1);

                let safe_with_remove =
                    is_safe_level_2(&first_removed, Some(i)) || is_safe_level_2(&second_removed, Some(i + 1));
                return safe_with_remove;
            } else {
                return false;
            }
        }

        let diff = first.abs_diff(second);
        if !(1..=3).contains(&diff) {
            if skip.is_none() {
                let mut first_removed = report.to_vec();
                first_removed.remove(i);
                let mut second_removed = report.to_vec();
                second_removed.remove(i+1);

                let safe_with_remove =
                    is_safe_level_2(&first_removed, Some(i)) || is_safe_level_2(&second_removed, Some(i + 1));
                return safe_with_remove;
            } else {
                return false;
            }
        }
        if first.cmp(&second) != valid_ordering {
            if skip.is_none() {
                let mut first_removed = report.to_vec();
                first_removed.remove(i);
                let mut second_removed = report.to_vec();
                second_removed.remove(i+1);

                let safe_with_remove =
                    is_safe_level_2(&first_removed, Some(i)) || is_safe_level_2(&second_removed, Some(i + 1));
                return safe_with_remove;
            } else {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_safe_level_2(&[7, 6, 4, 2, 1], None));
        assert!(!is_safe_level_2(&[1, 2, 7, 8, 9], None));
        assert!(!is_safe_level_2(&[9, 7, 6, 2, 1], None));
        assert!(is_safe_level_2(&[1, 3, 2, 4, 5], None));
        assert!(is_safe_level_2(&[8, 6, 4, 4, 1], None));
        assert!(is_safe_level_2(&[1, 3, 6, 7, 9], None));
    }
}

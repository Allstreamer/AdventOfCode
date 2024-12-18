use std::collections::HashMap;

pub fn solve_task_one(file: &str) -> usize {
    let (limits, manuals) = file.split_once("\n\n").unwrap();

    let limits = limits
        .lines()
        .map(|limit| limit.split_once("|").unwrap())
        .map(|limit| {
            (
                limit.0.parse::<usize>().unwrap(),
                limit.1.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let manuals = manuals
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    manuals
        .iter()
        .filter(|man| {
            let mut map = HashMap::new();
            for (i, item) in man.iter().enumerate() {
                map.insert(*item, i);
            }
            for rule in &limits {
                if let (Some(a), Some(b)) = (map.get(&rule.0), map.get(&rule.1)) {
                    if a > b {
                        return false;
                    }
                }
            }
            true
        })
        .map(|x| get_center_value(x))
        .sum()
}

fn get_center_value(manual: &[usize]) -> usize {
    manual[manual.len() / 2]
}

pub fn solve_task_two(file: &str) -> usize {
    let (limits, manuals) = file.split_once("\n\n").unwrap();

    let limits = limits
        .lines()
        .map(|limit| limit.split_once("|").unwrap())
        .map(|limit| {
            (
                limit.0.parse::<usize>().unwrap(),
                limit.1.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let mut manuals = manuals
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|man| {
            let mut map = HashMap::new();
            for (i, item) in man.iter().enumerate() {
                map.insert(*item, i);
            }
            for rule in &limits {
                if let (Some(a), Some(b)) = (map.get(&rule.0), map.get(&rule.1)) {
                    if a > b {
                        return true;
                    }
                }
            }
            false
        })
        .collect::<Vec<Vec<usize>>>();

    manuals
        .iter_mut()
        .map(|man| {
            man.sort_by(|a, b| {
                for rule in &limits {
                    if rule.0 == *a && rule.1 == *b {
                        return std::cmp::Ordering::Less;
                    }
                    if rule.0 == *b && rule.1 == *a {
                        return std::cmp::Ordering::Greater;
                    }
                }
                // abc
                std::cmp::Ordering::Equal
            });
            man
        })
        .map(|x| get_center_value(x))
        .sum()
}

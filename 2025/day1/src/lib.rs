pub fn solve_task_one(file: &str) -> u64 {
    let mut list_one = vec![];
    let mut list_two = vec![];

    for line in file.trim().lines() {
        let mut line_parts = line.split_ascii_whitespace();
        list_one.push(line_parts.next().unwrap().parse::<u64>().unwrap());
        list_two.push(line_parts.next().unwrap().parse::<u64>().unwrap());
    }

    list_one.sort();
    list_two.sort();

    list_one
        .iter()
        .zip(list_two.iter())
        .map(|(one, two)| one.abs_diff(*two))
        .sum::<u64>()
}

pub fn solve_task_two(file: &str) -> u64 {
    let mut list_one = vec![];
    let mut list_two = vec![];

    for line in file.trim().lines() {
        let mut line_parts = line.split_ascii_whitespace();
        list_one.push(line_parts.next().unwrap().parse::<u64>().unwrap());
        list_two.push(line_parts.next().unwrap().parse::<u64>().unwrap());
    }

    list_one
        .iter()
        .map(|list_one_item| {
            list_one_item
                * list_two
                    .iter()
                    .filter(|list_two_item| list_one_item == *list_two_item)
                    .count() as u64
        })
        .sum::<u64>()
}

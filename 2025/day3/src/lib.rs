use regex::Regex;

pub fn solve_task_one(file: &str) -> usize {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let result = regex.find_iter(file);
    result.map(|mat| mat.as_str().split(",")).map(|mat| {
        let mut mat_clone = mat.clone();
        mat_clone.next().unwrap().replace("mul(", "").parse::<usize>().unwrap() *
        mat_clone.next().unwrap().replace(")", "").parse::<usize>().unwrap()
    }).sum::<usize>()
}

pub fn solve_task_two(file: &str) -> usize {
    let regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    let result = regex.find_iter(file);
    let mut filted_multiplications_list  = vec![];
    let mut is_do = true;
    for res in result {
        let result_str = res.as_str();
        if result_str == "do()" {
            is_do = true;
            continue;
        }
        if result_str == "don't()" {
            is_do = false;
            continue;
        }
        if !is_do {
            continue;
        }
        filted_multiplications_list.push(result_str.split(","));
    }
    filted_multiplications_list.iter().map(|mat| {
        let mut mat_clone = mat.clone();
        mat_clone.next().unwrap().replace("mul(", "").parse::<usize>().unwrap() *
        mat_clone.next().unwrap().replace(")", "").parse::<usize>().unwrap()
    }).sum::<usize>()
}

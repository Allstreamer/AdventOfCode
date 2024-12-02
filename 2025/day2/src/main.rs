use day2::{solve_task_one, solve_task_two};

fn main() {
    //println!("{}", day2::is_safe_level_2(&[1, 2, 3, 4, 5], None));
    println!(
        "Result Task One: {}",
        solve_task_one(include_str!("../input_1.txt"))
    );
    println!(
        "Result Task Two: {}",
        solve_task_two(include_str!("../input_1.txt"))
    );
}

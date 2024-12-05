fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day4::solve_task_one(divan::black_box(include_str!("../input_1.txt")));
}

#[divan::bench]
fn part2() {
    day4::solve_task_two(divan::black_box(include_str!("../input_1.txt")));
}

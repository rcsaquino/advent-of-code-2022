use std::fs;
mod solutions;
fn main() {
    solutions::day_one::print_answer(fs::read_to_string("./src/inputs/day_one.txt").unwrap());
    solutions::day_two::print_answer(fs::read_to_string("./src/inputs/day_two.txt").unwrap());
    solutions::day_three::print_answer(fs::read_to_string("./src/inputs/day_three.txt").unwrap());
}

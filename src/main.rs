use std::fs;
mod solutions;
fn main() {
    solutions::day_01::print_answer(fs::read_to_string("./src/inputs/day_01.txt").unwrap());
    solutions::day_02::print_answer(fs::read_to_string("./src/inputs/day_02.txt").unwrap());
    solutions::day_03::print_answer(fs::read_to_string("./src/inputs/day_03.txt").unwrap());
}

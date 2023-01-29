use std::fs;
mod solutions;
fn main() {
    solutions::day_one::print_answer(fs::read_to_string("./src/inputs/day_one.txt").unwrap());
}

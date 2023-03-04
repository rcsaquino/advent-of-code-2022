use std::fs;
mod solutions;

fn main() {
    solutions::day_01::print_answer(fs::read_to_string("./src/inputs/day_01.txt").unwrap());
    solutions::day_02::print_answer(fs::read_to_string("./src/inputs/day_02.txt").unwrap());
    solutions::day_03::print_answer(fs::read_to_string("./src/inputs/day_03.txt").unwrap());
    solutions::day_04::print_answer(fs::read_to_string("./src/inputs/day_04.txt").unwrap());
    solutions::day_05::print_answer(fs::read_to_string("./src/inputs/day_05.txt").unwrap());
    solutions::day_06::print_answer(fs::read_to_string("./src/inputs/day_06.txt").unwrap());
    solutions::day_07::print_answer(fs::read_to_string("./src/inputs/day_07.txt").unwrap());
}

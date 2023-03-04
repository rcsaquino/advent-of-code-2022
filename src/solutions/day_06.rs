pub fn print_answer(input: String) {
    println!("---DAY 6---");
    println!("Part 1: {}", get_marker(&input, 4));
    println!("Part 2: {}", get_marker(&input, 14));
}

fn get_marker(input: &str, count: usize) -> usize {
    for i in 0..input.len() {
        let chunk: Vec<char> = input[i..=i + count].chars().collect();
        let mut is_signal = true;
        'outer: for x in 0..count - 1 {
            for y in x + 1..count {
                if chunk[x] == chunk[y] {
                    is_signal = false;
                    break 'outer;
                }
            }
        }
        if is_signal {
            return i + count;
        }
    }
    return 0;
}

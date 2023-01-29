pub fn print_answer(input: String) {
    // Part 1
    let mut priority_sum: u32 = 0;
    input.split("\r\n").for_each(|rucksack| {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);

        for c in first.chars() {
            if second.contains(c) {
                priority_sum += get_priority(c);
                break;
            }
        }
    });
    println!("---DAY 3---");
    println!("Part 1: {priority_sum}");

    // Part 2
    priority_sum = 0;
    let rucksacks: Vec<&str> = input.split("\r\n").collect();

    for i in (0..rucksacks.len()).step_by(3) {
        for c in rucksacks[i].chars() {
            if rucksacks[i + 1].contains(c) && rucksacks[i + 2].contains(c) {
                priority_sum += get_priority(c);
                break;
            }
        }
    }
	println!("Part 2: {priority_sum}");
}

fn get_priority(letter: char) -> u32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let lower_case_vec: Vec<char> = alphabet.chars().collect();
    let upper_case_vec: Vec<char> = alphabet.to_uppercase().chars().collect();

    let priority;
    if lower_case_vec.contains(&letter) {
        priority = lower_case_vec
            .iter()
            .position(|character| character == &letter)
            .unwrap()
            + 1;
    } else {
        priority = upper_case_vec
            .iter()
            .position(|character| character == &letter)
            .unwrap()
            + 27;
    }
    return priority as u32;
}

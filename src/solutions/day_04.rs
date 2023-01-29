fn to_vector(range: (&str, &str)) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for x in (range.0.parse::<u8>().unwrap())..=(range.1.parse::<u8>().unwrap()) {
        result.push(x)
    }
    return result;
}

pub fn print_answer(input: String) {
    let mut fully_count: u32 = 0;
    let mut overlap_count: u32 = 0;
    input.split("\r\n").for_each(|pair_range| {
        let (first, second) = pair_range.split_once(",").unwrap();

        let first_set = to_vector(first.split_once("-").unwrap());
        let second_set = to_vector(second.split_once("-").unwrap());

        // Part 1
        if first_set.iter().all(|x| second_set.contains(x))
            || second_set.iter().all(|x| first_set.contains(x))
        {
            fully_count += 1;
        }

        // Part 2
        if first_set.iter().any(|x| second_set.contains(x)) {
            overlap_count += 1;
        }
    });
    println!("---DAY 4---");
    println!("Part 1: {fully_count}");
    println!("Part 2: {overlap_count}");
}

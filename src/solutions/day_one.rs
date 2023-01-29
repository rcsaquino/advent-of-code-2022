pub fn print_answer(input: String) {
    let mut total_vec: Vec<u32> = Vec::new();

    input.split("\r\n\r\n").for_each(|elf| {
        let mut current: u32 = 0;
        elf.split("\r\n").for_each(|calorie| {
            current += calorie.parse::<u32>().unwrap();
        });
        total_vec.push(current)
    });
	
    total_vec.sort_by(|a, b| b.cmp(a));

    println!("Day 1 (Part 1): {:?}", total_vec[0]);
    println!("Day 1 (Part 2): {}", total_vec.iter().take(3).sum::<u32>())
}

pub fn print_answer(input: String) {
    let mut stacks_1: [Vec<char>; 9] = Default::default();
    let mut stacks_2: [Vec<char>; 9] = Default::default();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    for (i, line) in input.split("\r\n").enumerate() {
        // Stacks
        if i < 8 {
            for (i, c) in line.chars().enumerate() {
                if alphabet.contains(c) {
                    stacks_1[(i - 1) / 4].push(c)
                }
            }
        }

        // Free line. Use to reverse vectors.
        if i == 8 {
            for x in 0..stacks_1.len() {
                stacks_1[x].reverse()
            }

            stacks_2 = stacks_1.clone();
        }

        // Instructions
        if i > 9 {
            let instructions: Vec<&str> = line.split(' ').collect();
            let count = instructions[1].parse::<usize>().unwrap();
            let from = instructions[3].parse::<usize>().unwrap() - 1;
            let to = instructions[5].parse::<usize>().unwrap() - 1;

            // Part 1
            for _ in 0..count {
                let item = stacks_1[from].pop().unwrap();
                stacks_1[to].push(item)
            }

            // Part 2
            let items: Vec<char> = stacks_2[from].drain((stacks_2[from].len() - count)..).collect();
            stacks_2[to].extend(items);
        }
    }

    let mut result_1 = String::new();
    stacks_1.iter().for_each(|stack| {
        result_1.push(stack[stack.len() - 1]);
    });

    println!("---DAY 5---");
    println!("Part 1: {result_1:?}");

    let mut result_2 = String::new();
    stacks_2.iter().for_each(|stack| {
        result_2.push(stack[stack.len() - 1]);
    });

    println!("Part 2: {result_2:?}");
}

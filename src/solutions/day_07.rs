pub fn print_answer(input: String) {
    let mut directories: Vec<(String, u32)> = Vec::new();
    let mut path_vector: Vec<String> = Vec::new();

    // Go through each of the lines
    for line in input.split("\r\n").collect::<Vec<&str>>() {
        // We can skip $ ls and dir;
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        // 2 scenarios at this point: A "$ cd" or a file with size
        let line_chunks: Vec<&str> = line.split(" ").collect();

        if line.starts_with("$ cd") {
            // line_chunks[2] is the args after "$ cd"
            match line_chunks[2] {
                // Handle going back
                ".." => {
                    // Remove from path vactor
                    path_vector.pop().unwrap();
                }
                // Handle going forward; any dirs
                _ => {
                    let mut current_path = String::new();

                    // If not in the root folder, consider path_vector to be populated
                    if line_chunks[2] != "/" {
                        current_path = path_vector.join("_");
                        current_path += "_";
                    }
                    current_path += line_chunks[2];

                    // Push to path history
                    path_vector.push(current_path.clone());

                    // If it does not yet exists in directories db, push it with default size of 0
                    if !directories.iter().any(|(path, _)| path == &current_path) {
                        directories.push((current_path, 0));
                    }
                }
            }
        }
        // Files start with numerical character
        if line.chars().collect::<Vec<char>>()[0].is_numeric() {
            // If path is in path vector, then consider it current and add filesize
            for (path, size) in directories.iter_mut() {
                if path_vector.contains(path) {
                    *size += line_chunks[0].parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("---DAY 7---");

    // Part 1
    let mut sum = 0;
    for (_, size) in directories.clone() {
        if size <= 100_000 {
            sum += size;
        }
    }
    println!("Part 1: {sum}");

    // Part 2
    let capacity = 70_000_000;
    let update_size = 30_000_000;
    let space_needed = update_size - (capacity - directories[0].1);

    // temporarily set to capacity
    let mut smallest = capacity;

    for (_, size) in directories {
        if size >= space_needed && size < smallest {
            smallest = size
        }
    }
    println!("Part 2: {smallest}");
}

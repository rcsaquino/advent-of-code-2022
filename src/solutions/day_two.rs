enum Result {
    Win,
    Lose,
    Draw,
}
enum Shape {
    Rock,
    Paper,
    Scissors,
}

pub fn print_answer(input: String) {
	// Part 1
	let mut score: u32 = 0;
    input.split("\r\n").for_each(|scenario| match scenario {
        "A X" => score = update_score(score, Result::Draw, Shape::Rock),
        "A Y" => score = update_score(score, Result::Win, Shape::Paper),
        "A Z" => score = update_score(score, Result::Lose, Shape::Scissors),

        "B X" => score = update_score(score, Result::Lose, Shape::Rock),
        "B Y" => score = update_score(score, Result::Draw, Shape::Paper),
        "B Z" => score = update_score(score, Result::Win, Shape::Scissors),

        "C X" => score = update_score(score, Result::Win, Shape::Rock),
        "C Y" => score = update_score(score, Result::Lose, Shape::Paper),
        "C Z" => score = update_score(score, Result::Draw, Shape::Scissors),

        _ => {}
    });

	println!("Day 2 (Part 1): {score}");

	// Part 2
	score = 0;
	input.split("\r\n").for_each(|scenario| match scenario {
        "A X" => score = update_score(score, Result::Lose, Shape::Scissors),
        "A Y" => score = update_score(score, Result::Draw, Shape::Rock),
        "A Z" => score = update_score(score, Result::Win, Shape::Paper),

        "B X" => score = update_score(score, Result::Lose, Shape::Rock),
        "B Y" => score = update_score(score, Result::Draw, Shape::Paper),
        "B Z" => score = update_score(score, Result::Win, Shape::Scissors),

        "C X" => score = update_score(score, Result::Lose, Shape::Paper),
        "C Y" => score = update_score(score, Result::Draw, Shape::Scissors),
        "C Z" => score = update_score(score, Result::Win, Shape::Rock),

        _ => {}
    });

	println!("Day 2 (Part 2): {score}");
}

fn update_score(mut score: u32, result: Result, shape: Shape) -> u32 {
    match result {
        Result::Win => score += 6,
        Result::Draw => score += 3,
        Result::Lose => score += 0,
    }

    match shape {
        Shape::Rock => score += 1,
        Shape::Paper => score += 2,
        Shape::Scissors => score += 3,
    }

    return score;
}

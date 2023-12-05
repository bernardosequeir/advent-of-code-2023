advent_of_code::solution!(4);

pub struct GameCard {
    prize_numbers: Vec<u32>,
    card_numbers: Vec<u32>
}

impl GameCard {
    fn calculate_score(&self) -> u32 {
        let mut score = 0;

        for number in &self.prize_numbers {
            if self.card_numbers.contains(&number) {
                score = if score == 0 { 1 } else { score * 2}
            }
        }

        score
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let games = parse_input(input);
    
    for game in games {
        sum += game.calculate_score()
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn parse_input(input: &str) -> Vec<GameCard> {
    let mut cards: Vec<GameCard> = Vec::new();
    for line in input.lines() {
        let (id_with_trash, numbers_with_trash) = line.split_once(":").unwrap();
        let numbers_unparsed : Vec<&str> = numbers_with_trash.split("|").collect();

        let prize_numbers : Vec<u32> = parse_numbers(numbers_unparsed[0]);
        let card_numbers : Vec<u32> = parse_numbers(numbers_unparsed[1]);
                                        
       cards.push(GameCard { prize_numbers, card_numbers })
    }

    cards
}

pub fn parse_numbers(number_string : &str) -> Vec<u32> {
    number_string.split(" ").filter(|c| *c != "")
    .map(|u| u.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 13.into());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

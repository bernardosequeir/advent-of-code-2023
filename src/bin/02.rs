advent_of_code::solution!(2);

struct Game {
    id: u32,
    balls: Vec<(u32,u32,u32)>,
}

pub fn part_one(input: &str) -> Option<u32> {
    for line in input.lines() {
        println!("{}", line);
        let game = parse_game(line);
    }

    Some(1)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

pub fn parse_game(line: &str) -> Game {
    let gameId = line.chars()[line.find(":").unwrap() - 1 as usize];
    println!("{}", gameId); 
    Game { id: gameId, balls: [(1,1,1)].to_vec() }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 8.into());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

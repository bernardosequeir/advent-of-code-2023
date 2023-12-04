advent_of_code::solution!(2);

#[derive(Clone, Debug)]
pub struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_valid_game(&self, max_cubes : &GameSet) -> bool{
        for set in &self.sets {
            if set.green > max_cubes.green 
                || set.blue > max_cubes.blue 
                || set.red > max_cubes.red {
                    return false
                }    
        }

        return true
    }

    fn minimum_blocks_needed_power(&self) -> u32 { 
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in &self.sets {
            if set.red > max_red { 
                max_red = set.red;
            }
            if set.green > max_green { 
                max_green = set.green;
            }
            if set.blue > max_blue { 
                max_blue = set.blue;
            }
        }

        max_red * max_green * max_blue
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut id_sum = 0;
    let max_cubes = GameSet { green:  13, blue: 14, red: 12 };
    for line in input.lines() {
        let game: Game = parse_game(line);
        
        if game.is_valid_game(&max_cubes) {
            id_sum += game.id;
        }
    }

    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut power_sum = 0;
    for line in input.lines() {
        let game: Game = parse_game(line);
        power_sum += game.minimum_blocks_needed_power();
    }

    Some(power_sum)
}

pub fn parse_game(line: &str) -> Game {
    let (game_id_with_trash, unparsed_sets) = line.split_once(": ").unwrap();
    let parsed_id: u32 = game_id_with_trash.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
    let sets: Vec<&str> = unparsed_sets.split(";").collect();
    let mut parsed_sets: Vec<GameSet> = Vec::new(); 

    for set in sets {
        let colors : Vec<&str> = set.split(",").collect();
        let mut current_set = GameSet { green: 0, blue: 0, red: 0};

        for color in colors {
            let colors_split : Vec<&str> = color.trim().split(" ").collect();

            if color.ends_with("green") {
                current_set.green = colors_split[0].parse().unwrap();
            } else if color.ends_with("blue") {
                current_set.blue = colors_split[0].parse().unwrap();
            } else if color.ends_with("red") {
                current_set.red = colors_split[0].parse().unwrap();
            }
        }

        parsed_sets.push(current_set);
    }

    Game { id: parsed_id, sets: parsed_sets }
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
        assert_eq!(result, 2286.into());
    }
}

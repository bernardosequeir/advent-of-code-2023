use std::char;

advent_of_code::solution!(3);

#[derive(Debug)]
pub struct Coordinates {
    x: i32,
    y: i32
}
#[derive(Debug)]
pub struct Number {
    coordinates: Coordinates,
    length: i32,
    value: i32
}
#[derive(Debug)]
pub struct Symbol { 
    coordinates: Coordinates,
    value: char
}

impl Symbol {
    fn is_gear(&self) -> bool {
        self.value == '*'
    } 

    fn get_gear_ratio(&self, numbers: &Vec<Number>) -> i32 {
        if !self.is_gear() {
            return 0
        }

        let symbol_coords: &Coordinates = &self.coordinates;

        let mut adjacent_numbers : Vec<i32> = Vec::new();

        for number in numbers {
            let x = &number.coordinates.x;
            let y = &number.coordinates.y;
            let x_range: Vec<i32> = Vec::from([*x-1, *x, *x+1]);
            let y_range: Vec<i32> = ((y-1)..(y + &number.length + 1)).collect();
            if x_range.contains(&symbol_coords.x) && y_range.contains(&symbol_coords.y) {
                adjacent_numbers.push(number.value)
            }
        }

        if adjacent_numbers.len() < 2 {
            return 0
        }

        let mut gear_ratio = 1;

        for number in adjacent_numbers {
            gear_ratio *= number
        }

        gear_ratio
    }
}

impl Number { 
    fn has_adjacent_symbol(&self, symbols: &Vec<Symbol>) -> bool {
        let x = &self.coordinates.x;
        let y = &self.coordinates.y;
        let x_range: Vec<i32> = Vec::from([*x-1, *x, *x+1]);
        let y_range: Vec<i32> = ((y-1)..(y + &self.length + 1)).collect();

        for symbol in symbols {
            let symbol_coords = &symbol.coordinates;
            if x_range.contains(&symbol_coords.x) && y_range.contains(&symbol_coords.y) {
                return true;
            }
        }

        false 
    }
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for line in input.lines(){
        let mut current_number = Number { coordinates : Coordinates { x: 0, y: 0 }, length: 0, value : 0 };
        let mut is_appending = false;
        
        for c in line.chars(){
            if c.is_ascii_digit() {
                if !is_appending {
                    current_number.coordinates = Coordinates { x , y };
                    current_number.value = i32::try_from(c.to_digit(10).unwrap()).unwrap();
                    current_number.length += 1;
                    is_appending = true;
                } else {
                    current_number.value = current_number.value * 10 + i32::try_from(c.to_digit(10).unwrap()).unwrap();
                    current_number.length += 1;
                }
            } else {
                is_appending = false;
                if current_number.value != 0 {
                    numbers.push(current_number);
                }

                current_number = Number { coordinates : Coordinates { x: 0, y: 0 }, length: 0, value : 0 };
                
                if c != '.' {
                    symbols.push(Symbol { coordinates: Coordinates { x , y }, value: c })
                }
            }
            y += 1; 
        }

        if is_appending {
            numbers.push(current_number);
        }   

        x += 1;
        y = 0;
    }

    (numbers,symbols)
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: i32 = 0;
    let (numbers, symbols) = parse_input(input);

    for number in &numbers {
        if number.has_adjacent_symbol(&symbols) {
            sum += number.value;
        }
    }

    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: i32 = 0;
    let (numbers, symbols) = parse_input(input);

    for symbol in &symbols {
        sum += symbol.get_gear_ratio(&numbers);
    }

    Some(sum.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 4361.into());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 467835.into());
    }
}

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut numbers = line.chars().filter_map(|c| c.to_digit(10));

        let first_number = numbers.next().unwrap();
        let last_number = numbers.last().unwrap_or(first_number);
        
        sum += first_number * 10 + last_number;
    } 

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let mut sum = 0;

    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in input.lines() {
    
        let mut string = line.to_string();

        for (i, digit) in digits.iter().enumerate() {
            /* 
             Just replacing the string makes it so it won't 
             find other numbers if they used part of this number's chars,
             so I'm just adding the full number in chars before and after 
             the digit as not to mess with the other number themselves
             */
            
            let replace_string = digit.to_string() + (i + 1).to_string().as_str() + digit; 
            string = string.replace(digit, replace_string.as_str());
        }


        let mut numbers = string.chars().filter_map(|c| c.to_digit(10));

        
        let first_number = numbers.next().unwrap_or(0);
        let last_number = numbers.last().unwrap_or(first_number);

        sum += first_number * 10 + last_number; 
    } 

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 142.into());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, 281.into());
    }
}

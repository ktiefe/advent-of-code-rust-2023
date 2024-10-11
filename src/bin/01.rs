advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    let output= input.lines().map(|line| { 
        let mut it = line.chars().filter_map(|c|{c.to_digit(10)});

        dbg!(it.next());

        let first = it.next().expect("Should be number");
        dbg!(first);
        let last = it.last();

        dbg!(last);
        dbg!(match last{
            Some(num) => format!("{first}{num}").parse::<u32>(),
            None => format!("{first}{first}").parse::<u32>(),
        }).expect("Should be number")
    }).sum::<u32>();
    Some(output)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

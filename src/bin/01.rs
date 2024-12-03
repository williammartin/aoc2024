use std::collections::HashMap;
use std::iter::zip;

use nom::character::complete::{digit1, newline, space1};
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

advent_of_code::solution!(1);

#[derive(Debug)]
struct Lists(Vec<u32>, Vec<u32>);

fn lists(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many1(terminated(locations, newline))(input)
}

fn locations(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(location, space1, location)(input)
}

fn location(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_lists(input: &str) -> Lists {
    let parsed_lists = lists(input).expect("to nomnomnom successfully");
    let (l1, l2): (Vec<u32>, Vec<u32>) = parsed_lists.1.into_iter().unzip();
    Lists(l1, l2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lists = parse_lists(input);
    lists.0.sort();
    lists.1.sort();

    Some(zip(lists.0, lists.1).fold(0, |acc, e| acc + e.0.abs_diff(e.1)))
}

#[derive(Debug)]
struct Occurrences(HashMap<u32, u32>);

impl FromIterator<u32> for Occurrences {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut m = HashMap::new();

        for i in iter {
            if let Some(count) = m.get(&i) {
                m.insert(i, count + 1);
            } else {
                m.insert(i, 1);
            }
        }

        Occurrences(m)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lists = parse_lists(input);

    let left_occurences: Occurrences = lists.0.into_iter().collect();
    let right_occurences: Occurrences = lists.1.into_iter().collect();

    Some(left_occurences.0.into_iter().fold(0, |acc, e| {
        if let Some(right_count) = right_occurences.0.get(&e.0) {
            acc + e.0 * e.1 * right_count
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

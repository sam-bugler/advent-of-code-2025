use std::ops::{RangeInclusive};
use std::str::FromStr;

pub fn process(input: &str) -> usize {
    let mut count = 0;

    input
        .split(',')
        .filter_map(|s| IdRange::from_str(s).ok())
        .map(|i| i.range())
        .for_each(|range| {
            for i in range {
                if !is_valid_id(i) {
                    count += i
                }
            }
        });

    count
}

struct IdRange {
    from: usize,
    to: usize
}

impl IdRange {
    pub fn range(&self) -> RangeInclusive<usize> {
        self.from..=self.to
    }
}

impl FromStr for IdRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ids = s.split('-').collect::<Vec<&str>>();
        let mut iter = ids.into_iter();

        let from = iter.nth(0).expect("position 0 not found");
        let to = iter.nth(0).expect("position 1 not found");

        let from_usize = from.parse::<usize>().expect("failed to parse from");
        let to_usize = to.parse::<usize>().expect("failed to parse to");

        Ok(Self{from: from_usize, to: to_usize})
    }
}

fn is_valid_id(id: usize) -> bool {
    let id_str = id.to_string();
    let id_bytes = id_str.as_bytes();

    if !id_str.len().is_multiple_of(2) {
        return true
    }

    let chunk_length = id_str.len() / 2;

    let mut chunks = id_bytes.chunks(chunk_length);
    let a = chunks.nth(0).unwrap();
    let b = chunks.nth(0).unwrap();

    a != b
}

#[cfg(test)]
pub mod tests {
    use crate::part_1::process;

    #[test]
    fn process_works() {
        let test_case = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = process(test_case);
        let expected_result = 1227775554;

        assert_eq!(result, expected_result)
    }
}
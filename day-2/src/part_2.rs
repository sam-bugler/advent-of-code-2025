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

    let chunk_size_range = (1..id_str.len())
        .into_iter()
        .filter(|chunk_size| id_str.len().is_multiple_of(*chunk_size)) ;

    for chunk_size in chunk_size_range {
        let mut chunks = id_bytes.chunks(chunk_size);
        let mut is_uniform = true;

        let a = chunks.nth(0).expect("Should have at least one element");

        while let Some(b) = chunks.nth(0) {
           is_uniform &= a == b
        }

        if is_uniform {
            return false
        }
    }

   true
}

#[cfg(test)]
pub mod tests {
    use crate::part_2::process;

    #[test]
    fn process_works() {
        let test_case = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = process(test_case);
        let expected_result = 4174379265;

        assert_eq!(result, expected_result)
    }
}

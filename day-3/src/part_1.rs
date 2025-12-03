pub fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| PowerBank::from(line).max_joltage())
        .reduce(|acc, i| acc + i)
        .unwrap_or(0)
}

struct PowerBank<'a> {
    value: &'a str
}

impl<'a> PowerBank<'a> {
    pub fn max_joltage(&self) -> usize {
        let mut max = 0;
        let bytes = self.value.as_bytes();

        for x in 0..self.value.len() {
            for y in 0..self.value.len() {
                if y <= x {
                    continue;
                }

                let xy = format!("{}{}", bytes[x] as char, bytes[y] as char);
                let value = into_usize(&xy);

                if value > max {
                    max = value
                }
            }
        }

        max
    }
}

impl<'a> From<&'a str> for PowerBank<'a> {

    fn from(value: &'a str) -> Self {
        Self{value}
    }
}

fn into_usize(value: &str) -> usize {
    let bytes = value.as_bytes();

    let digits = bytes
        .iter()
        .map(|i|  (i - b'0') as usize );

    let multiples = (0..bytes.len())
        .rev()
        .map(|i| 10_usize.pow(i as u32));

    digits.zip(multiples)
        .map(|(digit, multiple)|  digit * multiple )
        .reduce(|acc, i| acc + i )
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::part_1::{into_usize, process, PowerBank};

    #[test]
    fn process_works() {
        let test_case =
            r"987654321111111
              811111111111119
              234234234234278
              818181911112111";

        let result = process(test_case);
        let expected_result = 357;

        assert_eq!(result, expected_result)
    }

    #[test]
    fn power_bank_max_joltage_works() {
        let test_cases = vec![
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92)
        ];

        for (test_case, expected_result) in test_cases {
            let power_bank = PowerBank::from(test_case);
            let result = power_bank.max_joltage();

            assert_eq!(result, expected_result, "result: {result}, expected: {expected_result}, test_case: {test_case}")
        }
    }

    #[test]
    fn into_usize_works() {
        let test_cases = vec![
            ("1234", 1234),
            ("123", 123),
            ("12", 12),
            ("1", 1)
        ];

        for (test_case, expected_result) in test_cases {
            let result = into_usize(test_case);
            assert_eq!(result, expected_result)
        }
    }
}
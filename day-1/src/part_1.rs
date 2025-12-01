use std::str::FromStr;

pub fn process(input: &str) -> usize {
    let mut output = 0;
    let mut dial = Dial::new(100, 50);

    input.lines()
        .map(|l| l.trim())
        .for_each(|l| {
            let rotation = Rotation::from_str(l).expect("Failed to parse rotation");
            dial.rotate(rotation);

            if dial.value() == 0 {
                output += 1
            }
        });

    output
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
enum Rotation {
    Left(u32),
    Right(u32),
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut  s = s.to_string();
        let direction = s.remove(0);
        let value = s.parse::<u32>().expect("Failed to parse u8");

        let rotation = match direction {
            'L' => Self::Left(value),
            'R' => Self::Right(value),
            _ => panic!("Input parse failed")
        };

        Ok(rotation)
    }
}

struct Dial {
    ptr: isize,
    len: usize
}

impl Dial {
    pub fn new(len: usize, initial_position: isize) -> Self{
        Self {
            len,
            ptr: initial_position
        }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left(val) => {
                if val == 0{return;}

                let ptr = self.ptr - val as isize;
                let len = self.len as isize;

                self.ptr = ptr.rem_euclid(len);

            }
            Rotation::Right(val) => {
                if val == 0{return;}

                let ptr = self.ptr + val as isize;
                let len = self.len as isize;

                self.ptr = ptr.rem_euclid(len);
            }
        }
    }

    pub fn value(&self) -> isize {
        self.ptr
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::part_1::{process, Dial, Rotation};
    use crate::part_1::Rotation::{Left, Right};

    #[test]
    pub fn process_works() {
        let test =
        r#" L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82"#;

        let result = process(test);
        let expected_result = 3;

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn rotation_from_str_works() {
        let test_cases = vec![
            ("L50", Left(50)),
            ("R0", Right(0)),
        ];

        for (case, expected) in test_cases {
            let result = Rotation::from_str(case).expect("Failed to parse rotation");
            assert_eq!(result, expected)
        }
    }

    #[test]
    pub fn dial_works() {
        let test_cases = vec![
            (Rotation::Left(0), 50),
            (Rotation::Left(10), 40),
            (Rotation::Right(10), 60),
            (Rotation::Left(50), 0),
            (Rotation::Left(51), 99),
            (Rotation::Right(49), 99),
            (Rotation::Right(50), 0)
        ];

        for (case, expected_result) in test_cases {
            let mut dial = Dial::new(100, 50);
            dial.rotate(case);

            let result = dial.value();
            assert_eq!(result, expected_result, "Failed at {:?}", case);
        }
    }
}
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct BinarySpace {
    min: i32,
    max: i32,
}

impl BinarySpace {
    fn width(&self) -> i32 {
        self.max - self.min
    }

    fn midpoint(&self) -> i32 {
        self.min + (self.width() / 2)
    }

    fn partition_lower(&self) -> BinarySpace {
        BinarySpace { min: self.min, max: self.midpoint() }
    }

    fn partition_higher(&self) -> BinarySpace {
        BinarySpace { min: self.midpoint(), max: self.max }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Seat {
    row: i32,
    col: i32,
}

impl Seat {
    pub fn id(&self) -> i32 {
        self.row * 8 + self.col
    }
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl FromStr for Seat {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row_str, col_str) = s.split_at(7);
        let row_num = row_str.chars().fold(BinarySpace { min: 0, max: 128 }, |space, c| {
            match c {
                'F' => space.partition_lower(),
                'B' => space.partition_higher(),
                _ => panic!(),
            }
        }).min;
        let col_num = col_str.chars().fold(BinarySpace { min: 0, max: 8 }, |space, c| {
            match c {
                'L' => space.partition_lower(),
                'R' => space.partition_higher(),
                _ => panic!(),
            }
        }).min;
        Ok(Seat { row: row_num, col: col_num })
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::{BinarySpace, ParseError, Seat};

    #[test]
    fn binary_space_width() {
        assert_eq!(BinarySpace { min: 0, max: 128 }.width(), 128);
        assert_eq!(BinarySpace { min: 127, max: 128 }.width(), 1);
        assert_eq!(BinarySpace { min: 96, max: 128 }.width(), 32);
    }

    #[test]
    fn binary_space_midpoint() {
        assert_eq!(BinarySpace { min: 0, max: 128 }.midpoint(), 64);
        assert_eq!(BinarySpace { min: 127, max: 128 }.midpoint(), 127);
        assert_eq!(BinarySpace { min: 96, max: 128 }.midpoint(), 112);
    }

    #[test]
    fn binary_space_partition_lower() {
        assert_eq!(BinarySpace { min: 0, max: 128 }.partition_lower(),
                   BinarySpace { min: 0, max: 64 });
        assert_eq!(BinarySpace { min: 96, max: 128 }.partition_lower(),
                   BinarySpace { min: 96, max: 112 });
    }

    #[test]
    fn binary_space_partition_higher() {
        assert_eq!(BinarySpace { min: 0, max: 128 }.partition_higher(),
                   BinarySpace { min: 64, max: 128 });
        assert_eq!(BinarySpace { min: 96, max: 128 }.partition_higher(),
                   BinarySpace { min: 112, max: 128 });
    }

    #[test]
    fn seat_from_str() -> Result<(), ParseError> {
        assert_eq!("FBFBBFFRLR".parse::<Seat>()?, Seat { row: 44, col: 5 });
        Ok(())
    }
}
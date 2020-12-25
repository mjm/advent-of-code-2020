use std::num::ParseIntError;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct Ring {
    cups: Vec<u8>,
    cursor: usize,
}

impl From<&[u8]> for Ring {
    fn from(s: &[u8]) -> Self {
        Ring { cups: s.into(), cursor: 0 }
    }
}

impl Ring {
    pub fn from(s: &str) -> Result<Self, ParseIntError> {
        let elems = s.chars().map(|c| c.to_string().parse::<u8>()).collect::<Result<Vec<u8>, ParseIntError>>()?;
        Ok((&elems[..]).into())
    }

    pub fn perform_move(&mut self) {
        let picked_up = self.remove(3);
        let dest = self.destination_index();
        self.insert(dest, picked_up);
        self.advance();
    }

    /// Removes the next `n` cups after the current cursor position, and returns them in a vector.
    fn remove(&mut self, n: usize) -> Vec<u8> {
        let start = self.cursor + 1;
        let range = start..((start + n).min(self.cups.len()));
        let num_removed = range.len();
        let mut removed: Vec<u8> = self.cups.drain(range).collect();
        if num_removed < n {
            let range = 0..(n - num_removed);
            let num_removed = range.len();
            removed.splice(removed.len()..removed.len(), self.cups.drain(range));
            self.cursor -= num_removed;
        }
        removed
    }

    /// Inserts the given `cups` after the cup at the given index.
    fn insert<I: IntoIterator<Item=u8>>(&mut self, i: usize, cups: I) {
        let range = (i + 1)..(i + 1);
        let prev_len = self.cups.len();
        self.cups.splice(range, cups);
        if i < self.cursor {
            self.cursor += self.cups.len() - prev_len;
        }
    }

    /// Moves the cursor forward by one cup.
    fn advance(&mut self) {
        self.cursor = (self.cursor + 1) % self.cups.len();
    }

    fn destination_index(&self) -> usize {
        let mut target_value = self.cups[self.cursor] - 1;
        if target_value == 0 {
            target_value = *self.cups.iter().max().unwrap();
        }

        loop {
            match self.cups.iter().find_position(|c| { **c == target_value }) {
                Some((i, _)) => { return i; }
                None => {
                    target_value -= 1;
                    if target_value == 0 {
                        target_value = *self.cups.iter().max().unwrap();
                    }
                }
            }
        }
    }

    pub fn cup_ordering(&self) -> String {
        let mut result = String::new();
        let (one_pos, _) = self.cups.iter().find_position(|c| **c == 1).unwrap();

        for cup in &self.cups[(one_pos+1)..] {
            result += &cup.to_string();
        }

        for cup in &self.cups[..one_pos] {
            result += &cup.to_string();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use crate::cups::Ring;

    #[test]
    fn from_string() -> Result<(), ParseIntError> {
        assert_eq!(Ring {
            cups: vec![3, 2, 4, 1, 5],
            cursor: 0,
        }, Ring::from("32415")?);
        assert_eq!(Ring {
            cups: vec![4, 6, 3, 5, 2, 8, 1, 7, 9],
            cursor: 0,
        }, Ring::from("463528179")?);

        Ok(())
    }

    #[test]
    fn advance() -> Result<(), ParseIntError> {
        let mut ring = Ring::from("32415")?;
        assert_eq!(0, ring.cursor);
        ring.advance();
        assert_eq!(1, ring.cursor);

        // test wrapping at the end
        ring.cursor = 4;
        ring.advance();
        assert_eq!(0, ring.cursor);

        Ok(())
    }

    #[test]
    fn remove() -> Result<(), ParseIntError> {
        let mut ring = Ring::from("32415")?;
        assert_eq!(vec![2, 4, 1], ring.remove(3));
        assert_eq!(vec![3, 5], ring.cups);
        assert_eq!(0, ring.cursor);

        // test wrapping behavior
        ring = Ring::from("32415")?;
        ring.cursor = 3;
        assert_eq!(vec![5, 3, 2], ring.remove(3));
        assert_eq!(vec![4, 1], ring.cups);
        assert_eq!(1, ring.cursor);

        ring = Ring::from("32415")?;
        ring.cursor = 4;
        assert_eq!(vec![3, 2, 4], ring.remove(3));
        assert_eq!(vec![1, 5], ring.cups);
        assert_eq!(1, ring.cursor);

        Ok(())
    }

    #[test]
    fn insert() -> Result<(), ParseIntError> {
        let mut ring = Ring::from("32415")?;
        ring.advance();
        let removed_cups = ring.remove(3);
        ring.insert(0, removed_cups);
        assert_eq!(vec![3, 4, 1, 5, 2], ring.cups);
        assert_eq!(4, ring.cursor);

        ring = Ring::from("32415")?;
        let removed_cups = ring.remove(3);
        ring.insert(1, removed_cups);
        assert_eq!(vec![3, 5, 2, 4, 1], ring.cups);
        assert_eq!(0, ring.cursor);

        Ok(())
    }

    #[test]
    fn destination_index() -> Result<(), ParseIntError> {
        let mut ring = Ring::from("32415")?;
        ring.remove(3);
        assert_eq!(1, ring.destination_index());

        ring = Ring::from("389154267")?;
        ring.remove(3);
        assert_eq!(3, ring.destination_index());

        Ok(())
    }
}
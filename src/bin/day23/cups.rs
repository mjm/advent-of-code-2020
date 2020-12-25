#[derive(Debug)]
pub struct Ring {
    next: Vec<usize>,
    cursor: usize,
}

impl From<&[usize]> for Ring {
    fn from(s: &[usize]) -> Self {
        let mut next = Vec::new();
        next.resize(s.len() + 1, 0);

        next[s[s.len()-1]] = s[0];
        for i in 1..s.len() {
            next[s[i-1]] = s[i];
        }

        Ring { next, cursor: s[0] }
    }
}

impl Ring {
    pub fn perform_move(&mut self) {
        let mut removed_elems = Vec::new();
        for _ in 0..3 {
            let elem = self.next[self.cursor];
            self.next[self.cursor] = self.next[elem];
            removed_elems.push(elem);
        }

        let mut target_value = self.cursor - 1;
        if target_value == 0 {
            target_value = self.next.len() - 1;
        }

        while removed_elems.contains(&target_value) {
            target_value -= 1;
            if target_value == 0 {
                target_value = self.next.len() - 1;
            }
        }

        let previous_end = self.next[target_value];
        self.next[target_value] = removed_elems[0];
        self.next[removed_elems[2]] = previous_end;

        self.cursor = self.next[self.cursor];
    }

    pub fn cup_ordering(&self) -> String {
        let mut result = String::new();

        let mut cur_val = self.next[1];
        while cur_val != 1 {
            result += &cur_val.to_string();
            cur_val = self.next[cur_val];
        }

        result
    }

    pub fn cups_after_one(&self) -> (usize, usize) {
        let first = self.next[1];
        let second = self.next[first];
        (first, second)
    }
}
#[derive(Clone)]
pub struct Simulator {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Simulator {
    pub fn new(s: &str) -> Self {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        Simulator {
            map: s.chars().filter(|c| *c != '\n').collect(),
            width,
            height,
        }
    }

    pub fn step(&mut self) -> usize {
        let mut changes: Vec<(usize, char)> = Vec::new();

        for (i, c) in self.map.iter().enumerate() {
            match *c {
                'L' => {
                    if self.count_occupied_neighbors(i) == 0 {
                        changes.push((i, '#'));
                    }
                },
                '#' => {
                    if self.count_occupied_neighbors(i) >= 4 {
                        changes.push((i, 'L'));
                    }
                },
                _ => {},
            }
        }

        for (i, c) in &changes {
            self.map[*i] = *c;
        }

        changes.len()
    }

    pub fn run_until_stable(&mut self) {
        loop {
            if self.step() == 0 {
                return
            }
        }
    }

    pub fn step2(&mut self) -> usize {
        let mut changes: Vec<(usize, char)> = Vec::new();

        for (i, c) in self.map.iter().enumerate() {
            match *c {
                'L' => {
                    if self.count_occupied_visible_seats(i) == 0 {
                        changes.push((i, '#'));
                    }
                },
                '#' => {
                    if self.count_occupied_visible_seats(i) >= 5 {
                        changes.push((i, 'L'));
                    }
                },
                _ => {},
            }
        }

        for (i, c) in &changes {
            self.map[*i] = *c;
        }

        changes.len()
    }

    pub fn run_until_stable2(&mut self) {
        loop {
            if self.step2() == 0 {
                return
            }
        }
    }

    pub fn num_occupied(&self) -> usize {
        self.map.iter().filter(|c| **c == '#').count()
    }

    fn count_occupied_neighbors(&self, i: usize) -> usize {
        self.neighbor_indices(i).iter().filter(|j| self.map[**j] == '#').count()
    }

    fn neighbor_indices(&self, i: usize) -> Vec<usize> {
        vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ].iter().filter_map(|(dx, dy)| self.adjacent_index(i, *dx, *dy)).collect()
    }

    fn count_occupied_visible_seats(&self, i: usize) -> usize {
        self.visible_seat_indices(i).iter().filter(|j| self.map[**j] == '#').count()
    }

    fn visible_seat_indices(&self, i: usize) -> Vec<usize> {
        vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ].iter().filter_map(|(dx, dy)| self.visible_seat_index(i, *dx, *dy)).collect()
    }

    fn adjacent_index(&self, i: usize, dx: i32, dy: i32) -> Option<usize> {
        let x = i % self.width;
        let y = i / self.width;

        let mut next_index = i;
        if dx < 0 {
            if (x as i32) + dx < 0 {
                return None;
            }
            next_index -= dx.abs() as usize;
        } else if dx > 0 {
            if x + (dx as usize) >= self.width {
                return None;
            }
            next_index += dx as usize;
        }

        if dy < 0 {
            if (y as i32) + dy < 0 {
                return None;
            }
            next_index -= (dy.abs() as usize) * self.width;
        } else {
            if y + (dy as usize) >= self.height {
                return None;
            }
            next_index += (dy as usize) * self.width;
        }

        Some(next_index)
    }

    fn visible_seat_index(&self, i: usize, dx: i32, dy: i32) -> Option<usize> {
        match self.adjacent_index(i, dx, dy) {
            Some(j) => match self.map[j] {
                '#' | 'L' => Some(j),
                '.' => self.visible_seat_index(j, dx, dy),
                c => panic!("Unexpected character {}", c),
            },
            None => None,
        }
    }
}

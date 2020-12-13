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

    pub fn num_occupied(&self) -> usize {
        self.map.iter().filter(|c| **c == '#').count()
    }

    fn count_occupied_neighbors(&self, i: usize) -> usize {
        self.neighbor_indices(i).iter().filter(|j| self.map[**j] == '#').count()
    }

    fn neighbor_indices(&self, i: usize) -> Vec<usize> {
        let x = i % self.width;
        let y = i / self.width;

        let mut indices: Vec<usize> = Vec::new();

        let can_go_left = x > 0;
        let can_go_right = x < self.width - 1;
        let can_go_up = y > 0;
        let can_go_down = y < self.height - 1;

        if can_go_up {
            if can_go_left {
                indices.push(i - (self.width + 1));
            }
            indices.push(i - self.width);
            if can_go_right {
                indices.push(i - (self.width - 1));
            }
        }

        if can_go_left {
            indices.push(i - 1);
        }
        if can_go_right {
            indices.push(i + 1);
        }

        if can_go_down {
            if can_go_left {
                indices.push(i + self.width - 1);
            }
            indices.push(i + self.width);
            if can_go_right {
                indices.push(i + self.width + 1);
            }
        }

        indices
    }
}

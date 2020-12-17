use std::collections::HashMap;

fn main() {
    let input = [1, 20, 11, 6, 12, 0];

    part1(&input[..]);
    part2(&input[..]);
}

fn part1(input: &[i32]) {
    let mut game = Game::new();
    for num in input {
        game.speak(*num);
    }

    let result = game.simulate_until(2020);
    println!("The 2020th number spoken is {}", result);
}

fn part2(input: &[i32]) {
    let mut game = Game::new();
    for num in input {
        game.speak(*num);
    }

    let result = game.simulate_until(30000000);
    println!("The 30000000th number spoken is {}", result);
}

struct Game {
    turn: i32,
    last_turns: HashMap<i32, i32>,
    last_spoken: i32,
}

impl Game {
    fn new() -> Self {
        Game {
            turn: 0,
            last_turns: HashMap::new(),
            last_spoken: -1,
        }
    }

    fn speak(&mut self, num: i32) {
        if self.last_spoken >= 0 {
            self.last_turns.insert(self.last_spoken, self.turn);
        }
        self.last_spoken = num;
        self.turn += 1;
    }

    fn next_number(&self) -> i32 {
        match self.last_turns.get(&self.last_spoken) {
            Some(turn) => self.turn - turn,
            None => 0,
        }
    }

    fn simulate_until(&mut self, turn: i32) -> i32 {
        while self.turn < turn {
            self.speak(self.next_number());
        }

        self.last_spoken
    }
}
mod combat;

use advent_of_code_2020::read_input;
use crate::combat::{Player, Game};

fn main() {
    let contents = read_input();
    let players = Player::from_list(&contents).unwrap();

    part1(players);
}

fn part1(players: Vec<Player>) {
    let mut game = Game::new(players);
    let winner = game.play();

    println!("Winner is {} with score {}", winner.num, winner.score());
}
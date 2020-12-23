use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;
use nom::{Finish, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
}

#[derive(Debug)]
pub struct RecursiveGame {
    players: Vec<Player>,
    previous_rounds: HashSet<Vec<Player>>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Player {
    pub num: u32,
    deck: Deck,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Deck {
    cards: VecDeque<u32>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        assert_eq!(players.len(), 2);

        Game { players }
    }

    pub fn play(&mut self) -> &Player {
        loop {
            self.take_turn();
            if self.winner().is_some() {
                break;
            }
        }

        self.winner().unwrap()
    }

    pub fn take_turn(&mut self) {
        let cards_played = self.players.iter_mut()
            .enumerate()
            .map(|(idx, player)| {
                (idx, player.play_card().unwrap())
            })
            .collect_vec();

        let (winner, winner_card) = cards_played.iter().max_by_key(|(_, card)| card).unwrap();

        self.players[*winner].take_card(*winner_card);
        for card in cards_played.iter().filter(|(idx, _)| idx != winner).map(|(_, card)| *card) {
            self.players[*winner].take_card(card);
        }
    }

    pub fn winner(&self) -> Option<&Player> {
        if self.players.iter().filter(|p| p.lost()).count() != self.players.len() - 1 {
            return None;
        }

        self.players.iter().filter(|p| !p.lost()).next()
    }
}

impl RecursiveGame {
    pub fn new(players: Vec<Player>) -> Self {
        assert_eq!(players.len(), 2);

        RecursiveGame { players, previous_rounds: HashSet::new() }
    }

    pub fn play(&mut self) -> &Player {
        loop {
            self.take_turn();
            if self.winner().is_some() {
                break;
            }
        }

        self.winner().unwrap()
    }

    pub fn take_turn(&mut self) {
        let this_round = self.players.iter().cloned().collect();
        self.previous_rounds.insert(this_round);

        let cards_played = self.players.iter_mut()
            .enumerate()
            .map(|(idx, player)| {
                (idx, player.play_card().unwrap())
            })
            .collect_vec();

        if cards_played.iter().all(|(idx, card)| self.players[*idx].num_cards() >= *card) {
            let new_players = cards_played.iter()
                .map(|(idx, card)| {
                    self.players[*idx].clone_top(*card)
                })
                .collect_vec();

            let mut new_game = RecursiveGame::new(new_players);
            let winner = new_game.play();
            let winner_idx = self.players.iter().position(|p| p.num == winner.num).unwrap();
            self.give_cards_to_winner(winner_idx, cards_played);
            return;
        }

        let (winner, _) = cards_played.iter().max_by_key(|(_, card)| card).unwrap();
        self.give_cards_to_winner(*winner, cards_played);
    }

    fn give_cards_to_winner(&mut self, winner: usize, mut cards_played: Vec<(usize, u32)>) {
        cards_played.sort_unstable_by(|(idx1, _), (idx2, _)| {
            if *idx1 == winner {
                Ordering::Less
            } else if *idx2 == winner {
                Ordering::Greater
            } else {
                idx1.cmp(idx2)
            }
        });
        for (_, card) in cards_played {
            self.players[winner].take_card(card);
        }
    }

    pub fn winner(&self) -> Option<&Player> {
        if self.previous_rounds.contains(&self.players) {
            return Some(&self.players[0]);
        }

        if self.players.iter().filter(|p| p.lost()).count() != self.players.len() - 1 {
            return None;
        }

        self.players.iter().filter(|p| !p.lost()).next()
    }
}

impl Player {
    pub fn from_list(s: &str) -> Result<Vec<Self>, nom::error::Error<&str>> {
        all_consuming(parse_players)(s).finish().map(|(_, players)| players)
    }

    fn num_cards(&self) -> u32 {
        self.deck.len() as u32
    }

    fn play_card(&mut self) -> Option<u32> {
        self.deck.pop_top()
    }

    fn take_card(&mut self, card: u32) {
        self.deck.push_bottom(card);
    }

    fn lost(&self) -> bool {
        self.deck.is_empty()
    }

    pub fn score(&self) -> u32 {
        self.deck.cards.iter()
            .rev()
            .enumerate()
            .map(|(i, card)| {
                ((i + 1) as u32) * (*card)
            })
            .sum()
    }

    fn clone_top(&self, n: u32) -> Player {
        Player {
            num: self.num,
            deck: self.deck.clone_top(n),
        }
    }
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn pop_top(&mut self) -> Option<u32> {
        self.cards.pop_front()
    }

    fn push_bottom(&mut self, card: u32) {
        self.cards.push_back(card)
    }

    fn clone_top(&self, n: u32) -> Deck {
        Deck {
            cards: self.cards.iter().cloned().take(n as usize).collect(),
        }
    }
}

fn parse_players(s: &str) -> IResult<&str, Vec<Player>> {
    separated_list1(tag("\n\n"), parse_player)(s)
}

fn parse_player(s: &str) -> IResult<&str, Player> {
    map(
        separated_pair(
            preceded(tag("Player "), parse_digit1),
            tag(":\n"),
            parse_deck,
        ),
        |(num, deck)| Player { num, deck },
    )(s)
}

fn parse_deck(s: &str) -> IResult<&str, Deck> {
    map(separated_list1(newline, parse_digit1), |cards| {
        Deck { cards: cards.into() }
    })(s)
}

fn parse_digit1(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(s)
}

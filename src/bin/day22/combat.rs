use std::collections::VecDeque;
use nom::{IResult, Finish};
use nom::combinator::{map_res, map, all_consuming};
use nom::character::complete::{digit1, newline};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, preceded};
use nom::bytes::complete::tag;
use itertools::Itertools;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
}

#[derive(Debug)]
pub struct Player {
    pub num: u32,
    deck: Deck,
}

#[derive(Debug)]
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

impl Player {
    pub fn from_list(s: &str) -> Result<Vec<Self>, nom::error::Error<&str>> {
        all_consuming(parse_players)(s).finish().map(|(_, players)| players)
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
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn pop_top(&mut self) -> Option<u32> {
        self.cards.pop_front()
    }

    fn push_bottom(&mut self, card: u32) {
        self.cards.push_back(card)
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

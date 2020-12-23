use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::iter::once;

fn main() {
    let mut decks: Vec<VecDeque<usize>> = vec![];
    let mut current_deck = VecDeque::new();
    for line in INPUT.lines() {
        if line.starts_with("Player") {
            continue;
        } else if line == "" {
            decks.push(current_deck.clone());
            current_deck.clear();
        } else {
            current_deck.push_back(line.parse().unwrap());
        }
    }
    decks.push(current_deck);
    let (mut player_a, mut player_b) = decks.into_iter().collect_tuple().unwrap();
    // end of parsing

    let mut seen_games = HashSet::new();
    let mut winner = None;
    while !player_a.is_empty() && !player_b.is_empty() {
        let this_game = (player_a, player_b).clone();
        if seen_games.contains(&this_game) {
            winner = Some(player_a);
            break;
        } else {
            seen_games.insert(this_game);
        }

        let (card_a, card_b) = (player_a.pop_front().unwrap(), player_b.pop_front().unwrap());
        if [(player_a, card_a), (player_b, card_b)]
            .iter()
            .all(|(player, card)| player.len() >= *card)
        {
            // recurse here
            todo!()
        }

        if card_a > card_b {
            player_a.push_back(card_a);
            player_a.push_back(card_b);
        } else if card_b > card_a {
            player_b.push_back(card_b);
            player_b.push_back(card_a);
        } else {
            panic!("not sure what to do if they're equal")
        }
    }

    let (winner,) = once(player_a)
        .chain(once(player_b))
        .filter(|d| !d.is_empty())
        .collect_tuple()
        .unwrap();

    dbg!(winner
        .iter()
        .rev()
        .zip(1..)
        .map(|(card_no, index)| card_no * index)
        .sum::<usize>());
}

const INPUT: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

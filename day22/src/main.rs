use itertools::Itertools;
use std::collections::VecDeque;
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
    while !player_a.is_empty() && !player_b.is_empty() {
        let (card_a, card_b) = (player_a.pop_front().unwrap(), player_b.pop_front().unwrap());
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
12
40
50
4
24
15
22
43
18
21
2
42
27
36
6
31
35
20
32
1
41
14
9
44
8

Player 2:
30
10
47
29
13
11
49
7
25
37
33
48
16
5
45
19
17
26
46
23
34
39
28
3
38";

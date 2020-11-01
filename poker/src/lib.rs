use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

const CARD_VALUE: &[&str] = &[
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum PokerRank {
    StraightFlush(u8),
    FourOfAKind(u8, u8),
    FullHouse(u8, u8),
    Flush(u8, u8, u8, u8, u8),
    Straight(u8),
    ThreeOfAKind(u8, u8, u8),
    TwoPair(u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    HighCard(u8, u8, u8, u8, u8),
}

impl PokerRank {
    fn value(&self) -> Vec<u8> {
        match self {
            PokerRank::StraightFlush(a) => vec![8, *a],
            PokerRank::FourOfAKind(a, b) => vec![7, *a, *b],
            PokerRank::FullHouse(a, b) => vec![6, *a, *b],
            PokerRank::Flush(a, b, c, d, e) => vec![5, *a, *b, *c, *d, *e],
            PokerRank::Straight(a) => vec![4, *a],
            PokerRank::ThreeOfAKind(a, b, c) => vec![3, *a, *b, *c],
            PokerRank::TwoPair(a, b, c) => vec![2, *a, *b, *c],
            PokerRank::OnePair(a, b, c, d) => vec![1, *a, *b, *c, *d],
            PokerRank::HighCard(a, b, c, d, e) => vec![0, *a, *b, *c, *d, *e],
        }
    }
}

impl Ord for PokerRank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value()
            .iter()
            .zip(other.value())
            .map(|(a, b)| a.cmp(&b))
            .filter(|result| *result != Ordering::Equal)
            .next()
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    value_index: u8,
    suit: char,
}

fn hand_value(hand: &str) -> PokerRank {
    let mut cards = hand
        .split_whitespace()
        .map(|card_str| {
            let (value, suit) = card_str.split_at(card_str.len() - 1);

            Card {
                value_index: CARD_VALUE
                    .iter()
                    .position(|card_value| *card_value == value)
                    .unwrap() as u8
                    + 1,
                suit: suit.chars().into_iter().next().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    cards.sort_by(|a, b| b.value_index.cmp(&a.value_index));

    let first_suit = cards.first().unwrap().suit;

    let is_flush = cards.iter().all(|card| card.suit == first_suit);

    let last_card_index = CARD_VALUE.len() as u8;

    let is_straight = cards.windows(2).all(|items| {
        let first_index = items.first().unwrap().value_index;
        let second_index = items.last().unwrap().value_index;

        (first_index - second_index) == 1 || (first_index == last_card_index && second_index == 4)
    });

    if is_straight
        && cards[0].value_index == last_card_index
        && cards.last().unwrap().value_index == 1
    {
        //demote an ace if it is a low straight
        cards[0].value_index = 0;
    }

    let value_counts = cards.iter().fold(HashMap::new(), |mut acc, item| {
        *acc.entry(item.value_index).or_insert(0) += 1;
        acc
    });

    let mut value_pairs = value_counts.iter().collect::<Vec<_>>();
    value_pairs.sort_by(|a, b| {
        if a.1 == b.1 {
            b.0.cmp(&a.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    let first_value_pair = value_pairs[0];
    let second_value_pair = value_pairs[1];

    if is_straight && is_flush {
        return PokerRank::StraightFlush(cards[0].value_index);
    }

    if *first_value_pair.1 == 4 {
        return PokerRank::FourOfAKind(*first_value_pair.0, *second_value_pair.0);
    }

    if *first_value_pair.1 == 3 && *second_value_pair.1 == 2 {
        return PokerRank::FullHouse(*first_value_pair.0, *second_value_pair.0);
    }

    if is_flush {
        return PokerRank::Flush(
            cards[0].value_index,
            cards[1].value_index,
            cards[2].value_index,
            cards[3].value_index,
            cards[4].value_index,
        );
    }

    if is_straight {
        return PokerRank::Straight(cards[0].value_index);
    }

    if *first_value_pair.1 == 3 {
        return PokerRank::ThreeOfAKind(
            *first_value_pair.0,
            *second_value_pair.0,
            *value_pairs[2].0,
        );
    }

    if *first_value_pair.1 == 2 && *second_value_pair.1 == 2 {
        return PokerRank::TwoPair(*first_value_pair.0, *second_value_pair.0, *value_pairs[2].0);
    }

    if *first_value_pair.1 == 2 {
        return PokerRank::OnePair(
            *first_value_pair.0,
            *second_value_pair.0,
            *value_pairs[2].0,
            *value_pairs[3].0,
        );
    }

    PokerRank::HighCard(
        cards[0].value_index,
        cards[1].value_index,
        cards[2].value_index,
        cards[3].value_index,
        cards[4].value_index,
    )
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let hand_values = hands.iter().fold(HashMap::new(), |mut acc, hand| {
        acc.insert(*hand, hand_value(*hand));
        acc
    });

    let max_hand = hand_values.values().max().unwrap();

    Some(
        hand_values
            .iter()
            .filter(|(_, hand_value)| **hand_value == *max_hand)
            .map(|(hand, _)| *hand)
            .collect::<Vec<_>>(),
    )
}

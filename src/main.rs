/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;
use itertools::Itertools;
use std::str::FromStr;
use regex::Regex;

mod tests;

#[derive(PartialEq, PartialOrd)]
enum PokerHand<'a> {
    HighCard(&'a str),
    OnePair(&'a str),
    TwoPair(&'a str),
    ThreeOfAKind(&'a str),
    Straight(&'a str),
    Flush(&'a str),
    FullHouse(&'a str),
    FourOfAKind(&'a str),
    StraightFlush(&'a str),
    FiveOfAKind(&'a str),
}

impl<'a> PokerHand<'a> {
    fn new(hand: &'a str) -> Self {
        let cards: Vec<PlayingCard> = hand.split_whitespace().map(|c| PlayingCard::new(c)).collect();
        if same_rank(cards.iter().collect::<Vec<&PlayingCard>>()) {
            return PokerHand::FiveOfAKind(hand);
        } else if is_sequence(cards.iter().map(|c| c.rank).collect()) && same_suit(&cards) {
            return PokerHand::StraightFlush(hand);
        } else if cards.iter().combinations(4).any(|combo| same_rank(combo)) {
            return PokerHand::FourOfAKind(hand);
        } else if full_house(&cards) {
            return PokerHand::FullHouse(hand);
        } else if same_suit(&cards) {
            return PokerHand::Flush(hand);
        } else if is_sequence(cards.iter().map(|c| c.rank).collect()) {
            return PokerHand::Straight(hand);
        } else if cards.iter().combinations(3).any(|combo| same_rank(combo)) {
            return PokerHand::ThreeOfAKind(hand);
        } else if two_pair(&cards) {
            return PokerHand::TwoPair(hand);
        } else if cards.iter().combinations(2).any(|combo| same_rank(combo)) {
            return PokerHand::OnePair(hand);
        }
        return PokerHand::HighCard(hand);
    }
}

/// Extracts a pair from the hand and returns the hand without the pair, if any pair is found.
fn hand_minus_pair(cards: &Vec<PlayingCard>) -> Option<Vec<PlayingCard>> {
    for (index, card_1) in cards.iter().enumerate() {
        let mut new_hand: Vec<&PlayingCard> = cards.iter().collect();
        new_hand.remove(index);
        for (index, card_2) in new_hand.iter().enumerate() {
            if card_1.rank == card_2.rank {
                new_hand.remove(index);
                return Some(new_hand.into_iter().map(|c| *c).collect::<Vec<PlayingCard>>())
            }
        }
    }
    None
}

/// Returns true if the given list of cards contains two pairs, false otherwise.
fn two_pair(cards: &Vec<PlayingCard>) -> bool {
    match hand_minus_pair(cards) {
        Some(new_hand) => {
            new_hand.iter().combinations(2).any(|combo| same_rank(combo))
        },
        None => false
    }
}

/// Returns true if the given list of cards is a full house, false otherwise.
fn full_house(cards: &Vec<PlayingCard>) -> bool {
    match hand_minus_pair(cards) {
        Some(new_hand) => {
            if same_rank(new_hand.iter().collect::<Vec<&PlayingCard>>()) {
                true
            } else {
                false
            }
        },
        None => false
    }
}

/// Returns true if the given list of cards is all of the same rank, false otherwise.
fn same_rank(cards: Vec<&PlayingCard>) -> bool {
    cards.iter().all(|card| card.rank == cards[0].rank)
}

/// Returns true if the given list of cards is all of the same suit, false otherwise.
fn same_suit(cards: &Vec<PlayingCard>) -> bool {
    cards.iter().all(|card| card.suit == cards[0].suit)
}

/// Returns true if the given list of cards represents a sequence (shuffled or not), false otherwise.
fn is_sequence(mut ranks: Vec<u8>) -> bool {
    if ranks.len() < 2 {
        return true
    }
    ranks.sort();
    for i in 1..ranks.len() {
        if ranks[i] - ranks[i-1] != 1 {
            if ranks.contains(&1) {
                // This hand is not a sequence, but we can set aces high and check again
                return is_sequence(ranks.into_iter().map(|r| match r {
                    2..=13 => r,
                    1 => 14,
                    _ => panic!("This should never happen")
                }).collect())
            } else {
                return false
            }
        }
    }
    return true
}

#[derive(Clone, Copy)]
struct PlayingCard {
    suit: char,
    rank: u8,
}

impl PlayingCard {
    fn new(data: &str) -> Self {
        let re = Regex::new(r"^([JQKA]|\d{1,2})([HDSC])$").unwrap_or_else(|_| panic!("'{}' is not a valid card", data));
        let captures = re.captures(data).unwrap_or_else(|| panic!("'{}' is not a valid card", data));
        let given_rank = captures.get(1).unwrap_or_else(|| panic!("'{}' is not a valid card", data)).as_str();
        let suit = captures.get(2).unwrap_or_else(|| panic!("'{}' is not a valid card", data)).as_str();
        let rank = match given_rank {
            "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" => u8::from_str(given_rank).unwrap(),
            "J" => 11_u8,
            "Q" => 12_u8,
            "K" => 13_u8,
            "A" => 1_u8,
            _ => panic!("'{given_rank}' is not a valid playing card rank")
        };
        PlayingCard {
            rank,
            suit: char::from_str(suit).unwrap(),
        }
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut sorted_hands: Vec<PokerHand> = hands.iter().map(|h| PokerHand::new(h)).collect();
    sorted_hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    // TODO need to be able to determine ties & tie-breakers
    // TODO by checking high cards, high pairs, high triplets, etc - see test cases
    sorted_hands.iter().filter(|h| std::mem::discriminant(*h) == std::mem::discriminant(&sorted_hands[sorted_hands.len()-1])).map(|h| match h {
        PokerHand::HighCard(v) => *v,
        PokerHand::OnePair(v) => *v,
        PokerHand::TwoPair(v) => *v,
        PokerHand::ThreeOfAKind(v) => *v,
        PokerHand::Straight(v) => *v,
        PokerHand::Flush(v) => *v,
        PokerHand::FullHouse(v) => *v,
        PokerHand::FourOfAKind(v) => *v,
        PokerHand::StraightFlush(v) => *v,
        PokerHand::FiveOfAKind(v) => *v
    }).collect()
}


fn main() {
    println!("Hello, world!");
}

use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
use std::str::FromStr;
use regex::Regex;

mod tests;

// TODO: Implement equality & inequality operators for PokerHand to make it easy to identify
// TODO winning hand(s). Base equality/ineqality off of category, then fallback to checking for
// TODO highest flush, triple, pair, and single card. Full rules on Wikipedia.


#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum PokerHandType {
    HighCard{ranks: [u8; 5]},
    OnePair{pair_rank: u8, kicker_ranks: [u8; 3]},
    TwoPair{pair_ranks: [u8; 2], kicker_rank: u8},
    ThreeOfAKind{triplet_rank: u8, kicker_ranks: [u8; 2]},
    Straight{highest_rank: u8},
    Flush{ranks: [u8; 5]},
    FullHouse{pair_rank: u8, triplet_rank: u8},
    FourOfAKind{common_rank: u8, kicker_rank: u8},
    StraightFlush{highest_rank: u8},
}

#[derive(Eq)]
struct PokerHand<'a> {
    input: &'a str,
    cards: HashSet<PlayingCard>,
    category: PokerHandType,
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.cards == other.cards
    }
}

// TODO: Finish me!

// impl<'a> PartialOrd for PokerHand<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match self.category.cmp(&other.category) {
//             Ordering::Equal => {
//                 // Break ties between hands of the same type
//                 match self.category {
//                     PokerHandType::StraightFlush => self.cards.iter().max().unwrap().partial_cmp(&other.cards.iter().max().unwrap()),
//                     PokerHandType::FourOfAKind => {
//                         match &self.category.common_rank.cmp(&other.category.common_rank) {
//                             Ordering::Equal => &self.category.other_rank.partial_cmp(&other.category.other_rank),
//                             _ => &self.category.common_rank.cmp(&other.category.common_rank)
//                         }
//                     },
//                     _ => Some(Ordering::Equal)
//                 }
//             },
//             _ => self.category.partial_cmp(&other.category)
//         }
//     }
// }

impl<'a> PokerHand<'a> {
    fn new(input: &'a str) -> Self {
        let cards: HashSet<PlayingCard> = input.split_whitespace().map(|c| PlayingCard::new(c)).collect();
        let ranks = ranks(&cards);
        let category: PokerHandType = if let Some(highest_rank) = straight_flush(&cards) {
            PokerHandType::StraightFlush {highest_rank}
        } else if let Some((common_rank, kicker_rank)) = four_of_a_kind(&cards) {
            PokerHandType::FourOfAKind {common_rank, kicker_rank}
        } else if let Some((pair_rank, triplet_rank)) = full_house(&cards) {
            PokerHandType::FullHouse {pair_rank, triplet_rank}
        } else if same_suit(&cards) {
            PokerHandType::Flush{ranks: ranks.try_into().unwrap() }
        } else if let Some(highest_rank) = is_sequence(&ranks) {
            PokerHandType::Straight{highest_rank}
        } else if let Some((triplet_rank, other_rank, kicker_rank)) = three_of_a_kind(&cards) {
            PokerHandType::ThreeOfAKind{triplet_rank, kicker_ranks: [other_rank, kicker_rank]}
        } else if let Some((pair_ranks, kicker_rank)) = two_pair(&cards) {
            PokerHandType::TwoPair{pair_ranks, kicker_rank}
        } else if let Some((pair_rank, kicker_ranks)) = one_pair(&cards) {
            PokerHandType::OnePair{pair_rank, kicker_ranks}
        } else {
            PokerHandType::HighCard{ranks: ranks.try_into().unwrap()}
        };
        return PokerHand {
            input,
            cards,
            category
        }
    }
}

/// Returns the ranks of the given cards
fn ranks(cards: &HashSet<PlayingCard>) -> Vec<u8> {
    cards.iter().map(|c| c.rank).collect()
}

/// If the hand contains 5 cards in which the ranks form a sequence and the suits match,
/// returns the highest rank in the hand.
fn straight_flush(cards: &HashSet<PlayingCard>) -> Option<u8> {
    if let Some(highest_rank) = is_sequence(&cards.iter().map(|c| c.rank).collect()) {
        if same_suit(&cards) {
            return Some(highest_rank)
        }
    }
    None
}

/// If the hand contains 4 cards of the same rank, returns a tuple
/// of the shared rank and the individual rank.
fn four_of_a_kind(cards: &HashSet<PlayingCard>) -> Option<(u8, u8)> {
    for card in cards.iter() {
        let mut remaining_cards = cards.clone();
        remaining_cards.remove(card);
        if let Some(quintuplet_rank) = same_rank(&Vec::from_iter(remaining_cards.into_iter())) {
            return Some((quintuplet_rank, card.rank))
        }
    }
    None
}

/// If the hand contains 5 cards which comprise a full house, returns a tuple containing the
/// rank of the pair and the rank of the triplet.
fn full_house(cards: &HashSet<PlayingCard>) -> Option<(u8, u8)> {
    if let Some((pair_rank, remaining_hand)) = hand_minus_pair(cards) {
        if let Some(triplet_rank) = same_rank(&remaining_hand.iter().map(|c| *c).collect::<Vec<PlayingCard>>()) {
            return Some((pair_rank, triplet_rank))
        }
    }
    None
}

/// If the hand contains 3 cards of the same type, returns a tuple of the shared rank and the individual ranks.
fn three_of_a_kind(cards: &HashSet<PlayingCard>) -> Option<(u8, u8, u8)> {
    for combo in cards.iter().map(|c| *c).combinations(3) {
        if let Some(triplet_rank) = same_rank(&combo) {
            let extra_cards: Vec<PlayingCard> = cards.iter().filter(|c| c.rank != triplet_rank).map(|c| *c).collect();
            return Some((triplet_rank, extra_cards.get(0).unwrap().rank, extra_cards.get(1).unwrap().rank))
        }
    }
    None
}

/// If the hand contains 2 pairs, returns the rank of each pair and the kicker rank.
fn two_pair(cards: &HashSet<PlayingCard>) -> Option<([u8; 2], u8)> {
    if let Some((pair_1_rank, remaining_cards)) = hand_minus_pair(cards) {
        for combo in remaining_cards.iter().map(|c| *c).combinations(2) {
            if let Some(pair_2_rank) = same_rank(&combo) {
                return Some(([pair_1_rank, pair_2_rank], cards.iter().filter(|c| ! vec![pair_1_rank, pair_2_rank].contains(&c.rank)).map(|c| *c).collect::<Vec<PlayingCard>>().get(0).unwrap().rank))
            }
        }
    }
    None
}

/// If the hand contains a pair, returns the pair rank and the other cards' ranks.
fn one_pair(cards: &HashSet<PlayingCard>) -> Option<(u8, [u8; 3])> {
    for combo in cards.iter().map(|c| *c).combinations(2) {
        if let Some(pair_rank) = same_rank(&combo) {
            return Some((pair_rank, cards.iter().filter(|c| c.rank != pair_rank).map(|c| c.rank).collect::<Vec<u8>>().try_into().unwrap()))
        }
    }
    None
}

/// If the hand contains a pair, returns the pair's rank and the remaining cards.
fn hand_minus_pair(cards: &HashSet<PlayingCard>) -> Option<(u8, HashSet<PlayingCard>)> {
    for combo in cards.iter().map(|c| *c).combinations(2) {
        if let Some(pair_rank) = same_rank(&combo) {
            return Some((pair_rank, cards.iter().filter(|c| c.rank != pair_rank).map(|c| *c).collect()))
        }
    }
    None
}


/// If the given list of cards is all of the same rank, returns the rank.
fn same_rank(cards: &Vec<PlayingCard>) -> Option<u8> {
    let rank = cards.iter().next().unwrap().rank;
    if cards.iter().all(|card| card.rank == rank) {
        return Some(rank)
    }
    None
}

/// Returns true if the given list of cards is all of the same suit, false otherwise.
fn same_suit(cards: &HashSet<PlayingCard>) -> bool {
    cards.iter().all(|card| card.suit == cards.iter().next().unwrap().suit)
}

/// Returns the highest rank in the hand if the given list of cards represents a sequence (shuffled or not)
fn is_sequence(ranks: &Vec<u8>) -> Option<u8> {
    let mut ranks = ranks.clone();
    match ranks.len() {
        0 => None,
        1 => Some(*ranks.get(0).unwrap()),
        _ => {
            ranks.sort();
            for i in 1..ranks.len() {
                if ranks[i] - ranks[i-1] != 1 {
                    if ranks.contains(&14) {
                        // This hand is not a sequence, but we can set aces low and check again
                        return is_sequence(&ranks.into_iter().map(|r| match r {
                            2..=13 => r,
                            14 => 1,
                            _ => panic!("This should never happen")
                        }).collect())
                    } else {
                        return None
                    }
                }
            }
            Some(ranks[ranks.len()-1])
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, Hash)]
struct PlayingCard {
    suit: char,
    rank: u8,
}

impl PartialOrd for PlayingCard {

    /// Compare cards by rank.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
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
            "A" => 14_u8,
            _ => panic!("'{given_rank}' is not a valid playing card rank")
        };
        PlayingCard {
            rank,
            suit: char::from_str(suit).unwrap(),
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut sorted_hands: Vec<PokerHand> = hands.iter().map(|h| PokerHand::new(h)).collect();
    sorted_hands.sort_by(|a, b| a.category.partial_cmp(&b.category).unwrap_or(Ordering::Equal));
    let potential_winning_hands: Vec<&PokerHand> = sorted_hands.iter().filter(|h| &h.category == &sorted_hands[sorted_hands.len()-1].category).collect();

    Vec::new()
}


fn main() {
    println!("Hello, world!");
}

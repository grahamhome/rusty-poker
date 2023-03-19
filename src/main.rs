use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;
use std::str::FromStr;
use regex::Regex;

mod tests;

// TODO: Store references to cards, not ranks, in hand type

// TODO: Write additional tests to demonstrate that card, hand, and hand type comparisons & ordering work correctly.

// TODO (after submission): Organize into 3 modules: PlayingCard, PokerHand, PokerHandType


#[derive(Eq, Debug, PartialOrd, Ord)]
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

impl PartialEq for PokerHandType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
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

impl <'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for PokerHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.category == other.category {
            return match (&self.category, &other.category) {
                (PokerHandType::StraightFlush {highest_rank}, PokerHandType::StraightFlush {highest_rank: other_highest_rank}) => {
                    println!("both straight flushes");
                    highest_rank.cmp(other_highest_rank)
                },
                (PokerHandType::FourOfAKind {common_rank, kicker_rank}, PokerHandType::FourOfAKind {common_rank: other_common_rank, kicker_rank: other_kicker_rank}) => {
                    let quintuplet_comparison = common_rank.cmp(other_common_rank);
                    match quintuplet_comparison {
                        Ordering::Equal => kicker_rank.cmp(other_kicker_rank),
                        _ => quintuplet_comparison
                    }
                },
                (PokerHandType::FullHouse {pair_rank, triplet_rank}, PokerHandType::FullHouse {pair_rank: other_pair_rank, triplet_rank: other_triplet_rank}) => {
                    println!("full house, first pair rank: {}, first triplet rank: {}", pair_rank, triplet_rank);
                    println!("full house, second pair rank: {}, second triplet rank: {}", other_pair_rank, other_triplet_rank);
                    let triplet_comparison = triplet_rank.cmp(other_triplet_rank);
                    match triplet_comparison {
                        Ordering::Equal => pair_rank.cmp(other_pair_rank),
                        _ => triplet_comparison
                    }
                },
                (PokerHandType::Flush {ranks}, PokerHandType::Flush {ranks: other_ranks}) => ranks.iter().sorted().cmp(other_ranks.iter().sorted()),
                (PokerHandType::Straight {highest_rank}, PokerHandType::Straight {highest_rank: other_highest_rank}) => highest_rank.cmp(other_highest_rank),
                (PokerHandType::ThreeOfAKind {triplet_rank, kicker_ranks}, PokerHandType::ThreeOfAKind {triplet_rank: other_triplet_rank, kicker_ranks: other_kicker_ranks}) => {
                    let triplet_comparison = triplet_rank.cmp(other_triplet_rank);
                    match triplet_comparison {
                        Ordering::Equal => kicker_ranks.iter().sorted().cmp(other_kicker_ranks.iter().sorted()),
                        _ => triplet_comparison
                    }
                },
                (PokerHandType::TwoPair {pair_ranks, kicker_rank}, PokerHandType::TwoPair {pair_ranks: other_pair_ranks, kicker_rank: other_kicker_rank}) => {
                    let pairs_comparison = pair_ranks.iter().sum::<u8>().cmp(&other_pair_ranks.iter().sum::<u8>());
                    match pairs_comparison {
                        Ordering::Equal => kicker_rank.cmp(other_kicker_rank),
                        _ => pairs_comparison
                    }
                },
                (PokerHandType::OnePair {pair_rank, kicker_ranks}, PokerHandType::OnePair {pair_rank: other_pair_rank, kicker_ranks:other_kicker_ranks}) => {
                    let pair_comparison = pair_rank.cmp(other_pair_rank);
                    match pair_comparison {
                        Ordering::Equal => kicker_ranks.iter().sorted().cmp(other_kicker_ranks.iter().sorted()),
                        _ => pair_comparison
                    }
                },
                (PokerHandType::HighCard {ranks}, PokerHandType::HighCard {ranks: other_ranks}) => {
                    ranks.iter().sorted().cmp(other_ranks.iter().sorted())
                }
                _ => panic!()
            }
        } else if self.category > other.category {
            return Ordering::Greater
        } else {
            return Ordering::Less
        }
    }
}

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

// TODO: Do comparisons between cards directly, not between ranks

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
/// TODO: have to find triplet first, else triplet members get picked up as pair!
fn full_house(cards: &HashSet<PlayingCard>) -> Option<(u8, u8)> {
    if let Some((pair_rank, remaining_hand)) = hand_minus_pair(cards) {
        if let Some(triplet_rank) = same_rank(&remaining_hand.iter().map(|c| *c).collect::<Vec<PlayingCard>>()) {
            return Some((pair_rank, triplet_rank))
        }
    }
    None
}

/// TODO to support both full_house() and three_of_a_kind() use cases, this method should return only playing card instances
/// TODO All other methods will then have to be updated to do the same.
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

#[derive(Clone, Copy, Eq, Hash, Debug)]
struct PlayingCard {
    suit: char,
    rank: u8,
}

impl PartialOrd for PlayingCard {

    /// Compare cards by rank.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayingCard {

    /// Compare cards by rank
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialEq for PlayingCard {
    /// Equal cards have equal ranks
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
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
    sorted_hands.sort();
    let max = sorted_hands.iter().max().unwrap();
    sorted_hands.iter().filter(|h| *h == max).map(|h| h.input).collect()
}


fn main() {
    println!("Hello, world!");
    // TODO demo loop to accept hands and display winners
}

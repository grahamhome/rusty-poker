use std::cmp::Ordering;
use itertools::Itertools;
use std::str::FromStr;
use regex::Regex;

mod tests;

// TODO: Write additional tests to demonstrate that card, hand, and hand type comparisons & ordering work correctly.

// TODO (after submission): Organize into 3 modules: PlayingCard, PokerHand, PokerHandType


#[derive(Eq, Debug, PartialOrd, Ord)]
enum PokerHandType {
    HighCard{cards: Vec<PlayingCard>},
    OnePair{pair: Vec<PlayingCard>, kickers: Vec<PlayingCard>},
    TwoPair{high_pair: Vec<PlayingCard>, low_pair: Vec<PlayingCard>, kicker: PlayingCard},
    ThreeOfAKind{triplet: Vec<PlayingCard>, kickers: Vec<PlayingCard>},
    Straight{cards: Vec<PlayingCard>},
    Flush{cards: Vec<PlayingCard>},
    FullHouse{triplet: Vec<PlayingCard>, pair: Vec<PlayingCard>},
    FourOfAKind{quintuplet: Vec<PlayingCard>, kicker: PlayingCard},
    StraightFlush{cards: Vec<PlayingCard>},
}

impl PartialEq for PokerHandType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Eq)]
struct PokerHand<'a> {
    input: &'a str,
    category: PokerHandType,
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.category == other.category {
            return match self.category.cmp(&other.category) {
                Ordering::Equal => true,
                _ => false
            }
        }
        false
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
                (PokerHandType::StraightFlush {cards: hand_a}, PokerHandType::StraightFlush {cards: hand_b}) => hand_a.cmp(hand_b),
                (PokerHandType::FourOfAKind {quintuplet: quint_a, kicker: kicker_a}, PokerHandType::FourOfAKind {quintuplet: quint_b, kicker: kicker_b}) => {
                    let quintuplet_comparison = quint_a.cmp(quint_b);
                    match quintuplet_comparison {
                        Ordering::Equal => kicker_a.cmp(kicker_b),
                        _ => quintuplet_comparison
                    }
                },
                (PokerHandType::FullHouse {triplet: triplet_a, pair: pair_a}, PokerHandType::FullHouse {triplet: triplet_b, pair: pair_b}) => {
                    let triplet_comparison = triplet_a.cmp(triplet_b);
                    match triplet_comparison {
                        Ordering::Equal => pair_a.cmp(pair_b),
                        _ => triplet_comparison
                    }
                },
                (PokerHandType::Flush {cards: hand_a}, PokerHandType::Flush {cards: hand_b}) => hand_a.cmp(hand_b),
                (PokerHandType::Straight {cards: hand_a}, PokerHandType::Straight {cards: hand_b}) => hand_a.cmp(hand_b),
                (PokerHandType::ThreeOfAKind {triplet: triplet_a, kickers: kickers_a}, PokerHandType::ThreeOfAKind {triplet: triplet_b, kickers: kickers_b}) => {
                    let triplet_comparison = triplet_a.cmp(triplet_b);
                    match triplet_comparison {
                        Ordering::Equal => kickers_a.cmp(kickers_b),
                        _ => triplet_comparison
                    }
                },
                (PokerHandType::TwoPair {high_pair: high_pair_a, low_pair: low_pair_a, kicker: kicker_a}, PokerHandType::TwoPair {high_pair: high_pair_b, low_pair: low_pair_b, kicker: kicker_b}) => {
                    let high_pair_comparison = high_pair_a.cmp(high_pair_b);
                    return match high_pair_comparison {
                        Ordering::Equal => {
                            let low_pair_comparison = low_pair_a.cmp(low_pair_b);
                            match low_pair_comparison {
                                Ordering::Equal => kicker_a.cmp(kicker_b),
                                _ => low_pair_comparison
                            }
                        },
                        _ => high_pair_comparison
                    }
                },
                (PokerHandType::OnePair {pair: pair_a, kickers: kickers_a}, PokerHandType::OnePair {pair: pair_b, kickers: kickers_b}) => {
                    let pair_comparison = pair_a.iter().next().unwrap().cmp(pair_b.iter().next().unwrap());
                    match pair_comparison {
                        Ordering::Equal => kickers_a.cmp(kickers_b),
                        _ => pair_comparison
                    }
                },
                (PokerHandType::HighCard {cards: hand_1}, PokerHandType::HighCard {cards: hand_b}) => hand_1.cmp(hand_b),
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
        let cards: Vec<PlayingCard> = input.split_whitespace().map(|c| PlayingCard::new(c)).sorted().collect();
        let category: PokerHandType = if let Some(cards) = straight_flush(&cards) {
            PokerHandType::StraightFlush {cards}
        } else if let Some((quintuplet, kickers)) = n_of_a_kind(&cards, 4) {
            PokerHandType::FourOfAKind {quintuplet, kicker: kickers.into_iter().next().unwrap()}
        } else if let Some((triplet, pair)) = full_house(&cards) {
            PokerHandType::FullHouse {triplet, pair}
        } else if same_suit(&cards) {
            PokerHandType::Flush{cards}
        } else if let Some(cards) = is_sequence(&cards) {
            PokerHandType::Straight{cards}
        } else if let Some((triplet, kickers)) = n_of_a_kind(&cards, 3) {
            PokerHandType::ThreeOfAKind{triplet, kickers}
        } else if let Some((high_pair, low_pair, kicker)) = two_pair(&cards) {
            PokerHandType::TwoPair{high_pair, low_pair, kicker}
        } else if let Some((pair, kickers)) = n_of_a_kind(&cards, 2) {
            PokerHandType::OnePair{pair, kickers}
        } else {
            PokerHandType::HighCard{cards}
        };
        return PokerHand {
            input,
            category
        }
    }
}

/// Returns true if the hand contains 5 cards in which the ranks form a sequence and the suits match
fn straight_flush(cards: &Vec<PlayingCard>) -> Option<Vec<PlayingCard>> {
    if let Some(cards) = is_sequence(cards) {
        return if same_suit(&cards) {
            Some(cards)
        } else {
            None
        }

    }
    None
}

/// If the hand contains 5 cards which comprise a full house, returns the triplet and the pair.
fn full_house(cards: &Vec<PlayingCard>) -> Option<(Vec<PlayingCard>, Vec<PlayingCard>)> {
    if let Some((triplets, other_cards)) = n_of_a_kind(cards, 3) {
        if same_rank(&other_cards) {
            return Some((triplets, other_cards))
        }
    }
    None
}

/// If the hand contains an n-sized set of cards of the same type, returns the group and the extra cards.
fn n_of_a_kind(cards: &Vec<PlayingCard>, n: usize) -> Option<(Vec<PlayingCard>, Vec<PlayingCard>)> {
    for hand in cards.iter().map(|c| *c).combinations(n) {
        if same_rank(&hand) {
            let extra_cards: Vec<PlayingCard> = cards.iter().filter(|c| *c != hand.iter().next().unwrap()).map(|c| *c).collect();
            return Some((hand, extra_cards))
        }
    }
    None
}

/// If the hand contains 2 pairs, returns the two pairs and the remaining cards.
fn two_pair(cards: &Vec<PlayingCard>) -> Option<(Vec<PlayingCard>, Vec<PlayingCard>, PlayingCard)> {
    if let Some((pair_1, remaining_cards)) = n_of_a_kind(cards, 2) {
        if let Some((pair_2, remaining_cards)) = n_of_a_kind(&remaining_cards, 2) {
            let pair_1_highest = pair_1.iter().next().unwrap() >= pair_2.iter().next().unwrap();
            return if pair_1_highest {
                Some((pair_1, pair_2, remaining_cards.into_iter().next().unwrap()))
            } else {
                Some((pair_2, pair_1, remaining_cards.into_iter().next().unwrap()))
            }
        }
    }
    None
}

/// Returns true f the given list of cards is all of the same rank.
fn same_rank(cards: &Vec<PlayingCard>) -> bool {
    cards.iter().all(|card| card == cards.iter().next().unwrap())
}

/// Returns true ff the given list of cards is all of the same suit.
fn same_suit(cards: &Vec<PlayingCard>) -> bool {
    cards.iter().all(|card| card.suit == cards.iter().next().unwrap().suit)
}

/// If the given list of cards represents a sequence, returns the cards.
fn is_sequence(cards: &Vec<PlayingCard>) -> Option<Vec<PlayingCard>> {
    let mut cards = cards.clone();
    match cards.len() {
        0 => None,
        1 => Some(cards),
        _ => {
            cards.sort();
            for i in 1..cards.len() {
                if cards[i].rank - cards[i-1].rank != 1 {
                    // This hand is not a sequence: check for aces
                    return if cards.iter().any(|card| card.rank == 14) {
                        // Set all aces low and check again (no sequence will contain both a high and a low ace)
                        is_sequence(&cards.into_iter().map(|card| match card.rank {
                            2..=13 => card,
                            14 => PlayingCard { rank: 1, suit: card.suit },
                            _ => panic!()
                        }).sorted().collect())
                    } else {
                        None
                    }
                }
            }
            Some(cards)
        }
    }
}

#[derive(Clone, Copy, Eq, Debug)]
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
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false
        }
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

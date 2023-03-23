use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::hand_type::PokerHandType;

#[derive(Eq)]
pub struct PokerHand<'a> {
    pub input: &'a str,
    ranks: HashMap<u8, u8>,
    suits: HashSet<char>,
    category: PokerHandType,
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category.cmp(&other.category) == Ordering::Equal
            && self.sorted_ranks() == other.sorted_ranks()
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for PokerHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let category_comparison = self.category.cmp(&other.category);
        match category_comparison {
            Ordering::Equal => {
                self.sorted_ranks().cmp(&other.sorted_ranks())
            }
            _ => category_comparison,
        }
    }
}

impl<'a> PokerHand<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut ranks = HashMap::new();
        let mut suits = HashSet::new();
        for card in input.split_whitespace() {
            let (rank, suit) = card.split_at(card.len() - 1);
            ranks
                .entry(match rank {
                    "A" => 14,
                    "K" => 13,
                    "Q" => 12,
                    "J" => 11,
                    _ => rank.parse::<u8>().unwrap(),
                })
                .and_modify(|rank_count| *rank_count += 1)
                .or_insert(1);
            suits.insert(char::from_str(suit).unwrap());
        }
        let mut hand = PokerHand {
            ranks,
            suits,
            category: PokerHandType::HighCard,
            input,
        };
        hand.categorize();
        hand
    }

    /// Assigns the correct category to the hand.
    fn categorize(&mut self) {
        self.category = if self.same_suit() && self.sequence() {
            PokerHandType::StraightFlush
        } else if self.n_of_a_kind(4) {
            PokerHandType::FourOfAKind
        } else if self.full_house() {
            PokerHandType::FullHouse
        } else if self.same_suit() {
            PokerHandType::Flush
        } else if self.sequence() {
            PokerHandType::Straight
        } else if self.n_of_a_kind(3) {
            PokerHandType::ThreeOfAKind
        } else if self.two_pair() {
            PokerHandType::TwoPair
        } else if self.n_of_a_kind(2) {
            PokerHandType::OnePair
        } else {
            PokerHandType::HighCard
        }
    }

    /// Returns true if all cards in the hand are of the same suit.
    fn same_suit(&self) -> bool {
        self.suits.len() == 1
    }

    /// Returns true if the cards in the hand form a sequence.
    fn sequence(&mut self) -> bool {
        if self.ranks.values().any(|&rank_count| rank_count > 1_u8) {
            return false;
        };
        let sorted_ranks: Vec<u8> = self.ranks.keys().cloned().sorted().collect();
        for index in 1..sorted_ranks.len() {
            if sorted_ranks[index] - sorted_ranks[index - 1] > 1 {
                if sorted_ranks.contains(&14_u8) {
                    let mut alternate_hand = PokerHand {
                        suits: self.suits.clone(),
                        ranks: self.ranks.clone(),
                        category: PokerHandType::HighCard,
                        input: self.input,
                    };
                    alternate_hand.aces_low();
                    if alternate_hand.sequence() {
                        self.aces_low();
                        return true;
                    }
                }
                return false;
            }
        }
        true
    }

    /// Sets the value of all aces in the hand to 1.
    fn aces_low(&mut self) {
        let ace_count = *self.ranks.get(&14_u8).unwrap();
        self.ranks
            .entry(14_u8)
            .and_modify(|rank_count| *rank_count -= ace_count);
        self.ranks
            .entry(1_u8)
            .and_modify(|rank_count| *rank_count += ace_count)
            .or_insert(ace_count);
        self.ranks.remove(&14_u8);
    }

    /// Returns true if the hand contains an n-sized set of cards of the same rank.
    fn n_of_a_kind(&self, n: u8) -> bool {
        self.ranks.values().any(|&rank_count| rank_count == n)
    }

    /// Returns true if the hand contains two pairs of cards of the same rank.
    fn two_pair(&self) -> bool {
        self.ranks.values().cloned().sorted().collect::<Vec<u8>>() == [1, 2, 2]
    }

    /// Returns true if the hand contains a triplet and a pair.
    fn full_house(&self) -> bool {
        self.ranks.values().cloned().sorted().collect::<Vec<u8>>() == [2, 3]
    }

    /// Returns ranks sorted by count, then by rank, in decreasing order.
    fn sorted_ranks(&self) -> Vec<u8> {
        self.ranks
            .iter()
            .sorted_by(|(&rank_1, &count_1), (&rank_2, &count_2)| {
                if count_1 == count_2 {
                    rank_2.cmp(&rank_1)
                } else {
                    count_2.cmp(&count_1)
                }
            })
            .map(|(&rank, _)| rank)
            .collect()
    }
}

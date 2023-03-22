use std::cmp::Ordering;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Debug)]
pub struct PlayingCard {
    pub suit: char,
    pub rank: u8,
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
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl PlayingCard {
    pub fn new(data: &str) -> Self {
        let re = Regex::new(r"^([JQKA]|\d{1,2})([HDSC])$")
            .unwrap_or_else(|_| panic!("'{}' is not a valid card", data));
        let captures = re
            .captures(data)
            .unwrap_or_else(|| panic!("'{}' is not a valid card", data));
        let given_rank = captures
            .get(1)
            .unwrap_or_else(|| panic!("'{}' is not a valid card", data))
            .as_str();
        let suit = captures
            .get(2)
            .unwrap_or_else(|| panic!("'{}' is not a valid card", data))
            .as_str();
        let rank = match given_rank {
            "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" => {
                u8::from_str(given_rank).unwrap()
            }
            "J" => 11_u8,
            "Q" => 12_u8,
            "K" => 13_u8,
            "A" => 14_u8,
            _ => panic!("'{given_rank}' is not a valid playing card rank"),
        };
        PlayingCard {
            rank,
            suit: char::from_str(suit).unwrap(),
        }
    }
}
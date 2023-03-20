use crate::playing_card::PlayingCard;

#[derive(Eq, Debug, PartialOrd, Ord)]
pub enum PokerHandType {
    HighCard {
        cards: Vec<PlayingCard>,
    },
    OnePair {
        pair: Vec<PlayingCard>,
        kickers: Vec<PlayingCard>,
    },
    TwoPair {
        high_pair: Vec<PlayingCard>,
        low_pair: Vec<PlayingCard>,
        kicker: PlayingCard,
    },
    ThreeOfAKind {
        triplet: Vec<PlayingCard>,
        kickers: Vec<PlayingCard>,
    },
    Straight {
        cards: Vec<PlayingCard>,
    },
    Flush {
        cards: Vec<PlayingCard>,
    },
    FullHouse {
        triplet: Vec<PlayingCard>,
        pair: Vec<PlayingCard>,
    },
    FourOfAKind {
        quintuplet: Vec<PlayingCard>,
        kicker: PlayingCard,
    },
    StraightFlush {
        cards: Vec<PlayingCard>,
    },
}

impl PartialEq for PokerHandType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
use crate::hand::PokerHand;
use itertools::Itertools;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let sorted_hands: Vec<PokerHand> = hands.iter().map(|h| PokerHand::new(h)).sorted().collect();
    let max = sorted_hands.iter().max().unwrap();
    sorted_hands
        .iter()
        .filter(|h| *h == max)
        .map(|h| h.input)
        .collect()
}

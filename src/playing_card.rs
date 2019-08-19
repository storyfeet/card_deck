//! # Playing Cards
//!
//! This module is primarily intended as a demo
//! though it should be quite functional for most uses.
//!
//! It uses a struct with static enums for the types as every type needed a number for it's use.
//!
//! Even Jokers are different, they are represented as 0 and 1
//!
//! Do send a PR/Feature Request if some feature is missing
//!

use crate::deck::{Deck, DeckBuilder};
use failure_derive::Fail;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
    Joker,
}
use self::Suit::*;

impl Suit {
    pub fn from_usize(u: usize) -> Option<Suit> {
        match u {
            0 => Some(Spades),
            1 => Some(Clubs),
            2 => Some(Hearts),
            3 => Some(Diamonds),
            4 => Some(Joker),
            _ => None,
        }
    }

    pub fn from_char(c: char) -> Option<Suit> {
        match c {
            's' | 'S' => Some(Spades),
            'c' | 'C' => Some(Clubs),
            'h' | 'H' => Some(Hearts),
            'd' | 'D' => Some(Diamonds),
            'j' | 'J' => Some(Joker),
            _ => None,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Spades => 'S',
            Hearts => 'H',
            Clubs => 'C',
            Diamonds => 'D',
            Joker => 'J',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PCard {
    pub suit: Suit,
    pub num: u8,
}

/// Getting a Deck_Builder gives you more control over the options on the deck
/// ```
/// use card_deck::playing_card::{pcard_deck_builder,PCard};
/// let mut dk = pcard_deck_builder(true).pre_shuffle(false).done();
/// assert_eq!(dk.draw_1(),Some(PCard::s(1)));
/// ```
pub fn pcard_deck_builder(jokers: bool) -> DeckBuilder<PCard> {
    let mut rvec = Vec::new();
    for suit in 0..4 {
        for val in 1..14 {
            rvec.push(PCard::new(Suit::from_usize(suit).unwrap(), val));
        }
    }
    if jokers {
        rvec.push(PCard::new(Joker, 0));
        rvec.push(PCard::new(Joker, 1));
    }
    DeckBuilder::new().draw_pile(rvec)
}

pub fn pcard_deck(jokers: bool) -> Deck<PCard> {
    pcard_deck_builder(jokers).done()
}

/// ToCVal Exists as a trait to make short constructors work with either u8 or char
///
/// ```
/// use card_deck::playing_card::PCard;
/// let c = PCard::s(11); //s for spades
/// let c2 = PCard::s('J');
/// assert_eq!(c,c2);
/// ```
pub trait ToCVal {
    fn to_card_val(self) -> u8;
}

impl ToCVal for u8 {
    fn to_card_val(self) -> u8 {
        self
    }
}

impl ToCVal for char {
    fn to_card_val(self) -> u8 {
        match self {
            'a' | 'A' => 1,
            't' | 'T' => 10,
            'j' | 'J' => 11,
            'q' | 'Q' => 12,
            'k' | 'K' => 13,
            '1'...'9' => (self as u8) - 48,
            _ => 0,
        }
    }
}

/// The basic Playing Card Type
impl PCard {
    pub fn new<CV: ToCVal>(suit: Suit, v: CV) -> PCard {
        PCard {
            suit: suit,
            num: v.to_card_val(),
        }
    }

    pub fn s<CV: ToCVal>(v: CV) -> PCard {
        PCard::new(Spades, v)
    }
    pub fn c<CV: ToCVal>(v: CV) -> PCard {
        PCard::new(Clubs, v)
    }
    pub fn h<CV: ToCVal>(v: CV) -> PCard {
        PCard::new(Hearts, v)
    }
    pub fn d<CV: ToCVal>(v: CV) -> PCard {
        PCard::new(Diamonds, v)
    }
    pub fn jk(v: u8) -> PCard {
        PCard::new(Joker, v)
    }
}

#[derive(Clone, Debug, Copy,PartialEq, Fail)]
pub enum ParseErr {
    #[fail(display = "Could not parse Card")]
    BadParse,
    #[fail(display = "Input ran out")]
    TooShort,
}

impl FromStr for PCard {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<PCard, ParseErr> {
        let mut it = s.chars();

        let c =  it.next().ok_or(ParseErr::TooShort)?;
        let mut num = c.to_card_val();

        let mut next = it.next().ok_or(ParseErr::TooShort)?;

        // handle 10
        if c == '1' {
            if next == '0'{
                num = 10;
                next = it.next().ok_or(ParseErr::TooShort)?;
            }
        }
        // handle Jk
        if c == 'J' {
            if next == 'k' {
                return Ok(PCard::jk(1))
            }
        }

        let suit =
            Suit::from_char(next).ok_or(ParseErr::BadParse)?;

        Ok(PCard { suit, num })
    }
}

impl Display for PCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.suit == Joker {
            return write!(f, "Jk");
        }

        let c = match self.num {
            1 => 'A',
            2...9 => (self.num + 48) as char,
            10 => return write!(f, "10{}", self.suit.as_char()),
            11 => 'J',
            12 => 'Q',
            13 => 'K',
            _ => 'E',
        };

        write!(f, "{}{}", c, self.suit.as_char())
    }
}

#[cfg(test)]
mod test_playing_cards {
    use super::*;
    #[test]
    pub fn test_pcard_from_str() {
        assert_eq!(PCard::from_str("10S").unwrap(), PCard::s(10));
        assert_eq!(PCard::from_str("1S").unwrap(), PCard::s(1),"1 spades");
        assert_eq!(PCard::from_str("5H").unwrap(), PCard::h(5),"5 hearts");
        assert_eq!(PCard::from_str("Jk").unwrap(), PCard::jk(1));
        assert_eq!(PCard::from_str("JS").unwrap(), PCard::s(11));
        assert_eq!(PCard::from_str("JH").unwrap(), PCard::h('j'));
    }
}

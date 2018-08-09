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


use deck::{Deck,DeckBuilder};

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Suit{
    Spades,
    Clubs,
    Hearts,
    Diamonds,
    Joker,
}
use self::Suit::*;

impl Suit{
    pub fn from_usize(u:usize)->Option<Suit>{
        match u {
            0=>Some(Spades),
            1=>Some(Clubs),
            2=>Some(Hearts),
            3=>Some(Diamonds),
            4=>Some(Joker),
            _=>None,
        }
    }

    pub fn from_char(c:char)->Option<Suit>{
        match c{
            's'|'S'=>Some(Spades),
            'c'|'C'=>Some(Clubs),
            'h'|'H'=>Some(Hearts),
            'd'|'D'=>Some(Diamonds),
            'j'|'J'=>Some(Joker),
            _=>None
        }

    }
}


#[derive(Debug,Clone,Copy,PartialEq)]
pub struct PCard {
    pub suit:Suit,
    pub num:u8,
}

/// Getting a Deck_Builder gives you more control over the options on the deck
/// ```
/// use card_deck::playing_card::{pcard_deck_builder,PCard};
/// let mut dk = pcard_deck_builder(true).pre_shuffle(false).done();
/// assert_eq!(dk.draw_1(),Some(PCard::h(1)));
/// ```
pub fn pcard_deck_builder(jokers:bool)->DeckBuilder<PCard>{
    let mut rvec = Vec::new();
    for suit in 0..4{
        for val in 1..14{
            rvec.push(PCard::new(Suit::from_usize(suit).unwrap(),val));
        }
    }
    if jokers {
        rvec.push(PCard::new(Joker,0));
        rvec.push(PCard::new(Joker,1));
    }
    DeckBuilder::new().draw_pile(rvec)
}

pub fn pcard_deck(jokers:bool)->Deck<PCard>{
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
pub trait ToCVal{
    fn to_card_val(self)->u8;
}

impl ToCVal for u8 {
    fn to_card_val(self)->u8{
        self
    }
}


impl ToCVal for char{
    fn to_card_val(self)->u8{
        match self {
            'a'|'A'=>1,
            't'|'T'=>10,
            'j'|'J'=>11,
            'q'|'Q'=>12,
            'k'|'K'=>13,
            '1'...'9'=>(self as u8) - 48,
            _=>0,
        }
    }
}

/// The basic Playing Card Type
impl PCard {
    pub fn new<CV:ToCVal>(suit:Suit,v:CV)->PCard{
        PCard{
            suit:suit,
            num:v.to_card_val(),
        }
    }

    pub fn s<CV:ToCVal>(v:CV)->PCard{
        PCard::new(Spades,v)
    }
    pub fn c<CV:ToCVal>(v:CV)->PCard{
        PCard::new(Spades,v)
    }
    pub fn h<CV:ToCVal>(v:CV)->PCard{
        PCard::new(Spades,v)
    }
    pub fn d<CV:ToCVal>(v:CV)->PCard{
        PCard::new(Spades,v)
    }
    pub fn jk(v:u8)->PCard{
        PCard::new(Joker,v)
    }
}






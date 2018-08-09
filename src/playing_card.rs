
pub enum Suit{
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}
use self::Suit::*;

pub struct PCard {
    kind:Suit,
    num:u8,
}

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

impl PCard {
    pub fn new<CV:ToCVal>(kind:Suit,v:CV)->PCard{
        PCard{
            kind:kind,
            num:v.to_card_val(),
        }
    }

    pub fn s<CV:ToCVal>(v:u8)->PCard{
        PCard::new(Spades,v)
    }
    pub fn c<CV:ToCVal>(v:u8)->PCard{
        PCard::new(Spades,v)
    }
    pub fn h<CV:ToCVal>(v:u8)->PCard{
        PCard::new(Spades,v)
    }
    pub fn d<CV:ToCVal>(v:u8)->PCard{
        PCard::new(Spades,v)
    }
}

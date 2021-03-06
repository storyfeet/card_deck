//! Card Deck acts as a draw and discard pile to on any kind of card.
//!
//! **Contributions Welcome**
//!
//! It is defined Generically so as not to be limited to anything specific
//! like standard playing cards.
//!
//! It takes the attitude that cards, in general, cannot be copied. As such cards are not borrowed.
//! Instead they are consumed to be added, and when they are drawn, full ownership is returned.
//!
//! For some (digital) card games, cards can be copied.
//! If you wish to implement Clone, that will work in those cases.
//!

extern crate rand;

pub mod deck;
pub use deck::{Deck, DeckBuilder};

pub mod playing_card;

#[cfg(test)]
mod tests {
    use super::*;

    //new type to make sure Copy/Clone not required
    #[derive(Debug, PartialEq)]
    pub struct CTest(i32);

    #[test]
    fn noshuff() {
        let v = vec![CTest(0), CTest(1), CTest(2), CTest(3)];
        let mut dk = Deck::build().draw_pile(v).pre_shuffle(false).done();
        assert_eq!(dk.len(), 4);
        let mut max = 0;
        for (k, v) in dk.draw(4).enumerate() {
            assert_eq!(v, CTest(k as i32), "Enumerate in order failed");
            max = k;
        }
        assert_eq!(max, 3, "max eqauls total cards");
    }

    #[test]
    fn drawcards() {
        let v = vec![CTest(3), CTest(2), CTest(5), CTest(9), CTest(2)];
        let mut dk = Deck::new(v);

        assert_eq!(dk.len(), 5);

        let _ = dk.draw_1();
        //assert_eq!(c,Some(CTest(3)));
        assert_eq!(dk.len(), 4);

        let _ = dk.draw(2);

        assert_eq!(dk.len(), 2);
        for (k, c) in dk.draw(5).enumerate() {
            if k == 2 {
                panic!("Drew a third card {:?}", c);
            }
        }

        assert_eq!(dk.len(), 0);
    }
    #[test]
    fn test_dig() {
        let v = vec![CTest(0), CTest(1), CTest(2), CTest(3)];
        let mut dk = Deck::build().draw_pile(v).pre_shuffle(false).done();
        assert_eq!(dk.dig_for(|c| c == &CTest(1)), Some(CTest(1)));
        assert_eq!(dk.dig_for(|c| c == &CTest(4)), None);
        assert_eq!(dk.len(), 3);
    }

    #[test]
    fn test_dig_all() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut dk = Deck::build().draw_pile(v).pre_shuffle(false).done();
        let mut adder = 0;
        for c in dk.dig_all(|n| n % 2 != 0) {
            adder += c;
        }
        assert_eq!(dk.len(), 4);
        assert_eq!(adder, 16);

        let dr: Vec<u32> = dk.draw_all().collect();
        assert_eq!(vec![0, 2, 4, 6], dr);
    }
}

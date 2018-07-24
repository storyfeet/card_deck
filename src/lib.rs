//! Card Deck acts as a draw pile to on any kind of card.
//!
//! It is defined Generically so as not to be limited to anything specific
//! like standard playing cards.
//!
//! It takes the attitude that cards, in generall, cannot be copied. As such cards are not borrowed.
//! Instead they are consumed to be added, and when they are drawn, full ownership is returned.
//!
//! For some (digital) card games, cards can be copied.
//! If you wish to implement Clone, that will work in those cases.
//!

extern crate rand;

use std::vec::Drain;
use rand::Rng;


pub struct Deck<C>{
    draw_pile:Vec<C>,
    discard_pile:Vec<C>,
    shuffle_discards:bool,
    stop_on_discards:bool,
}

pub struct DeckBuilder<C>{
    draw_pile:Option<Vec<C>>,
    discard_pile:Option<Vec<C>>,
    shuffle_discards:bool,
    stop_on_discards:bool,
    pre_shuffle:bool,
}

impl<C> DeckBuilder<C>{
    pub fn new()->Self{
        DeckBuilder{
            draw_pile:None,
            discard_pile:None,
            pre_shuffle:true,
            shuffle_discards:true,
            stop_on_discards:false,
        }
    }
    pub fn draw_pile(mut self,v:Vec<C>)->Self{
        self.draw_pile = Some(v);
        self
    }
    pub fn discard_pile(mut self,v:Vec<C>)->Self{
        self.discard_pile = Some(v);
        self
    }
    pub fn pre_shuffle(mut self,b:bool)->Self{
        self.pre_shuffle = b;
        self
    }
    pub fn shuffle_discards(mut self,b:bool)->Self{
        self.shuffle_discards = b;
        self
    }
    
    pub fn stop_on_discards(mut self,b:bool)->Self{
        self.stop_on_discards = b;
        self
    }
            
            

    pub fn done(mut self)->Deck<C>{
        if self.pre_shuffle {
            if let Some(ref mut v) = self.draw_pile {
                rand::thread_rng().shuffle(v);
            }
            if let Some(ref mut v) = self.discard_pile {
                rand::thread_rng().shuffle(v);
            }
        }
        Deck{
            draw_pile:self.draw_pile.unwrap_or(Vec::new()),
            discard_pile:self.discard_pile.unwrap_or(Vec::new()),
            shuffle_discards:self.shuffle_discards,
            stop_on_discards:self.stop_on_discards,
        }

    }
}

impl<C> Deck<C>{
    ///Builds a deck using the supplied cards and defaults for all other options
    pub fn new(v:Vec<C>)->Self{
        Self::build().draw_pile(v).done()
    }

    ///Creates a Builder for the Deck, see DeckBuilder
    pub fn build()->DeckBuilder<C>{
        DeckBuilder::new()
    }

    ///Returns None if draw_pile is empty
    pub fn draw(&mut self)->Option<C>{
        return self.draw_n(1).next();
    }

    pub fn draw_n(&mut self,n:usize)->Drain<C>{
        if n > self.draw_pile.len(){
            return self.draw_pile.drain(0..)
        }
        self.draw_pile.drain(0..n)
    }

    ///returns the maximum number of cards that can be drawn in a single draw
    pub fn len(&self)->usize{
        match self.stop_on_discards {
            true => self.draw_pile.len(),
            false => self.draw_pile.len() + self.discard_pile.len(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    //new type to make sure Copy/Clone not required
    #[derive(Debug,PartialEq)]
    pub struct CTest(i32);
    

    #[test]
    fn noshuff(){
        let v = vec![CTest(0),CTest(1),CTest(2),CTest(3)];
        let mut dk = Deck::build().draw_pile(v).pre_shuffle(false).done();
        assert_eq!(dk.len(),4);
        let mut max = 0;
        for (k,v) in dk.draw_n(4).enumerate(){
            assert_eq!(v,CTest(k as i32),"Enumerate in order failed");
            max = k;
        }
        assert_eq!(max,3,"max eqauls total cards");
    }


    #[test]
    fn drawcards() {
        let v = vec![CTest(3),CTest(2),CTest(5),CTest(9),CTest(2)];
        let mut dk = Deck::new(v);
        
        assert_eq!(dk.len(),5);

        let _ = dk.draw();
        //assert_eq!(c,Some(CTest(3)));
        assert_eq!(dk.len(),4);

        let _ = dk.draw_n(2);

        assert_eq!(dk.len(),2);
        for (k,c) in dk.draw_n(5).enumerate(){
            if k == 2 {
                panic!("Drew a third card {:?}", c);
            }
        }

        assert_eq!(dk.len(),0);

    }
}

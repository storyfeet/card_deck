extern crate rand;

use std;
use std::vec::{Drain};
use rand::Rng;

///A Deck of cards
pub struct Deck<C>{
    draw_pile:Vec<C>,
    discard_pile:Vec<C>,
    shuffle_discards:bool,
    stop_on_discards:bool,
    chainer:Vec<C>,//Enables Chain to return something in IntoIter, 
}

/// A builder for a deck of cards. use "done" to finish.
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
    ///Fill the Draw Pile with the supplied vector, consuming it
    pub fn draw_pile(mut self,v:Vec<C>)->Self{
        self.draw_pile = Some(v);
        self
    }

    ///fill the Discard pile with the supplied vector consuming it
    pub fn discard_pile(mut self,v:Vec<C>)->Self{
        self.discard_pile = Some(v);
        self
    }

    /// Shuffle all cards, at constrution? 
    /// default: true
    pub fn pre_shuffle(mut self,b:bool)->Self{
        self.pre_shuffle = b;
        self
    }

    /// Shuffle discards before adding to the bottom of the draw_pile?
    /// default: true
    pub fn shuffle_discards(mut self,b:bool)->Self{
        self.shuffle_discards = b;
        self
    }
    
    /// If true, the deck is considered ending at the bottom of the draw pile.
    /// This is for all iterators and len
    /// Default: false
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
            chainer:Vec::new(),
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


    ///Add a card to the discard pile
    pub fn put_discard(&mut self,card:C){
        self.discard_pile.push(card);
    }

    /// Adds the Discard Pile to the bottom of the draw pile, shuffling if shuffle_discards
    /// ```
    /// use card_deck::Deck;
    /// let mut dk = Deck::build()
    ///             .draw_pile(vec![1,2,3])
    ///             .discard_pile(vec![4])
    ///             .stop_on_discards(true).done();
    ///
    /// assert_eq!(dk.len(),3);
    /// assert_eq!(dk.discard_len(),1);
    /// dk.discards_to_bottom();
    /// assert_eq!(dk.len(),4);
    /// assert_eq!(dk.discard_len(),0);
    ///
    /// assert_eq!(dk.draw_all().last(),Some(4));
    ///
    /// ```
    pub fn discards_to_bottom(&mut self){
        if self.shuffle_discards {
            rand::thread_rng().shuffle(&mut self.discard_pile);
        }
        self.draw_pile.append(&mut self.discard_pile);
    //    self.discard_pile = Vec::new();
    }

    pub fn shuffle_draw_pile(&mut self){
        rand::thread_rng().shuffle(&mut self.draw_pile);
    }

    ///Returns None if nothing to draw 
    pub fn draw_1(&mut self)->Option<C>{
        return self.draw(1).next();
    }

    pub fn draw(&mut self,n:usize)->Drain<C>{
        if n <= self.draw_pile.len() {
            return self.draw_pile.drain(0..n);
        }

        if self.stop_on_discards {
            return self.draw_pile.drain(0..n);
        }

        self.discards_to_bottom();
        if n <= self.draw_pile.len(){
            return self.draw_pile.drain(0..n)
        }
        self.draw_pile.drain(0..)
    }

    pub fn draw_all(&mut self)->Drain<C>{
        if ! self.stop_on_discards {
            self.discards_to_bottom();
        }
        self.draw_pile.drain(0..)
    }

    ///returns the maximum number of cards that can be drawn in a single draw
    pub fn len(&self)->usize{
        match self.stop_on_discards {
            true => self.draw_pile.len(),
            false => self.draw_pile.len() + self.discard_pile.len(),
        }
    }

    pub fn draw_len(&self)->usize{
        self.draw_pile.len()
    }
    pub fn discard_len(&self)->usize{
        self.discard_pile.len()
    }

    pub fn push_bottom(&mut self,c:C){
        self.draw_pile.push(c);
    }

    pub fn push_top(&mut self,c:C){
        self.discard_pile.insert(0,c);
    }

    pub fn push_discards(&mut self,c:C){
        self.discard_pile.push(c);
    }
  
    pub fn push_discards_top(&mut self,c:C){
        self.discard_pile.insert(0,c);
    }
}

/// Peeking cards
/// ```
/// use card_deck::Deck;
/// let dk = Deck::build().draw_pile(vec![1,2,3]).discard_pile(vec![4]).done();
/// let mut n = 0;
/// for c in &dk{
///     n += c;
/// }
/// assert_eq!(n,10);
///
/// //with stop_on_discards
///
/// let dk = Deck::build().draw_pile(vec![1,2,3]).discard_pile(vec![4])
///                 .stop_on_discards(true).done();
/// let mut n = 0;
/// for c in &dk{
///     n += c;
/// }
/// assert_eq!(n,6);
/// 
/// ```
impl<'a, C> IntoIterator for &'a Deck<C> 
    {
    type Item = &'a C;
    type IntoIter =  std::iter::Chain<std::slice::Iter<'a,C>,std::slice::Iter<'a,C>>;

    fn into_iter(self)->Self::IntoIter{
        match self.stop_on_discards{
            true=>(self.draw_pile).iter().chain(self.chainer.iter()),
            false=>(self.draw_pile).iter().chain(self.discard_pile.iter()),
        }
    }
}


/// Tweaking cards
/// ```
/// use card_deck::Deck;
/// let mut dk = Deck::build().draw_pile(vec![1,2,3]).pre_shuffle(false).done();
/// {
///     for c in &mut dk{
///       *c = *c +1;
///     }
/// }
///
/// assert_eq!(dk.draw_1(),Some(2));
/// ```
///
impl<'a, C> IntoIterator for &'a mut Deck<C>{
    type Item = &'a mut C;
    type IntoIter =  std::iter::Chain<std::slice::IterMut<'a,C>,std::slice::IterMut<'a,C>>;

    fn into_iter(self)->Self::IntoIter{
        match self.stop_on_discards{
            true=>(self.draw_pile).iter_mut().chain(self.chainer.iter_mut()),
            false=>(self.draw_pile).iter_mut().chain(self.discard_pile.iter_mut()),
        }
    }
}

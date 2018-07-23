use std::vec::Drain;
pub trait Card{
}

pub struct Deck<C:Card>{
    draw_pile:Vec<C>,
    discard_pile:Vec<C>,
}

impl<C:Card> Deck<C>{
    pub fn new(v:Vec<C>)->Self{
        Deck{
            draw_pile:v,
            discard_pile:Vec::new(),
        }
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

    pub fn len(&self)->usize{
        return self.draw_pile.len();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug,PartialEq)]
    pub struct CTest(i32);
    
    impl Card for CTest{
    }


    #[test]
    fn drawcards() {
        let mut v = vec![CTest(3),CTest(2),CTest(5),CTest(9),CTest(2)];
        let mut dk = Deck::new(v);
        
        assert_eq!(dk.len(),5);

        let c = dk.draw();
        assert_eq!(c,Some(CTest(3)));
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

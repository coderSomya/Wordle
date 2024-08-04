mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, guesser: G){                 //play 6 rounds where it invokes guesser each round
}


pub enum Correctness{
    //Green
    Correct,
    //Yellow
    Misplaced,
    //gray
    Wrong
}


pub struct Guess{
    pub word: String,
    pub mask: [Correctness; 5]
}

pub trait Guesser{
    fn guess(&mut self, history: &[Guess])-> String{}
}

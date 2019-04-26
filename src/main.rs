use rand::thread_rng;
use rand::seq::SliceRandom;

struct Deck<'a>{
    cards: Vec<&'a str>,
    current_index: usize,
}

impl<'a> Deck<'a> {
    fn shuffle(&mut self){
        self.cards.shuffle(&mut thread_rng());
    }

    fn deal_cards(&mut self, n: usize) -> Option<Vec<&str>> {
        if self.current_index > self.cards.len() - n {
            None
        } else {
            self.current_index += n;
            let mut v: Vec<&str> = Vec::new();
            v.extend_from_slice(&self.cards[self.current_index..self.current_index + n]);
            Some(v)
        }
    }
}
fn main() {
    let mut deck = Deck {
        cards: vec!["2s","2c","2h","2d","3s","3c","3h","3d","4s","4c","4h","4d","5s","5c","5h","5d",
                    "6s","6c","6h","6d","7s","7c","7h","7d","8s","8c","8h","8d","9s","9c","9h","9d",
                    "10s","10c","10h","10d","Js","Jc","Jh","Jd","Qs","Qc","Qh","Qd","Ks","Kc","Kh",
                    "Kd","As","Ac","Ah","Ad"],
        current_index: 0,
    };
    deck.shuffle();
    for card in deck.deal_cards(26).unwrap(){
        println!("{}", card);
    }
    for card in deck.deal_cards(26).unwrap(){
        println!("{}", card);
    }
}

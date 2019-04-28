use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;
fn build_table<'a>(deck: &'a [&Card], num_players: usize) -> Table<'a>{
    let new_deck: Vec<&Card> = deck.to_vec();
    Table {
        deck: new_deck,
        hole_cards: Vec::new(),
        streets: Vec::new(),
        cards_dealt: 0,
        num_players,
    }
}

struct Table <'a> {
    deck: Vec<&'a Card>,
    hole_cards: Vec<(&'a Card, &'a Card)>,
    streets: Vec<&'a Card>,
    cards_dealt: u8,
    num_players: usize
}

impl<'a> Table<'a> {
    fn shuffle(&mut self){
        self.deck.shuffle(&mut thread_rng());
    }

    fn deal_hole_cards(&mut self) {
        for _ in 0..self.num_players {
            self.hole_cards.push((self.deck.pop().expect("Out of cards"), (self.deck.pop().expect("Out of cards"))))
        }
    }
}

/*
 num represents card number
 2-10 equal their number, J = 11, Q = 12, K = 13, A = 14
*/
struct Card{
    suit: String,
    num: i8,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_num = match self.num {
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "T",
            11 => "J",
            12 => "Q",
            13 => "K",
            14 => "A",
            _ => "?",
        };
        write!(f, "{}{}", formatted_num, self.suit)
    }
}

fn main() {
    let cards = generate_cards();
    let cards_ref = cards.iter().collect::<Vec<_>>();
    let mut table = build_table(&cards_ref, 3);
    table.shuffle();
    for card in table.deck{
        println!("{}", card)
    }
}

fn generate_cards() -> Vec<Card>{
    let mut cards = Vec::new();
    for num in 2..15 {
        for suit in &["H", "D", "S", "C"]{
            cards.push(Card {suit: suit.to_string(), num});
        }
    }
    cards

}
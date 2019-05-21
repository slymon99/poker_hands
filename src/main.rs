use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;
use std::collections::HashMap;

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

    fn deal_full_game(&mut self) {
        self.deal_hole_cards();
        for _ in 0..5 {
            self.streets.push(self.deck.pop().expect("Out of cards"))
        }
    }
}

/*
 num represents card number
 2-10 equal their number, J = 11, Q = 12, K = 13, A = 14
*/
struct Card{
    suit: String,
    num: u8,
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
//    let cards = generate_cards();
//    let cards_ref = cards.iter().collect::<Vec<_>>();
//    let mut table = build_table(&cards_ref, 3);
//    table.shuffle();
//    table.deal_full_game();
//    for card in &table.deck{
//        println!("{}", card)
//    }
//    for card in &table.hole_cards{
//        println!("{} {}", card.0, card.1)
//    }
//    for card in &table.streets{
//        println!("{}", card)
//    }

    for hand in enumerate_unique_hands().keys(){
        println!();
        for group in hand{
            print!(" || ");
            for card in group{
                print!("{}-", card);
            }
        }
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

//here be dragons
fn enumerate_unique_hands() -> HashMap<Vec<Vec<u8>>, u32>{
    let mut res = HashMap::new();

    let cur_rank = 1;
    //enumerate all straight flushes
    for high_card in (6..15).rev() {
        for option in &[[7, 0, 0, 0], [6, 1, 0, 0], [5, 1, 1, 0], [5, 2, 0, 0]]{
            //calculate all potential groups
            let mut potential_groups = Vec::new();
            for (idx, group_total) in option.enumerate() {
                for excess_cards in enumerate_all_groups(
                    (0..high_card-4).collect().append(high_card + 1..15),
                    if idx == 0 {group_total - 5} else {group_total}){
                    let mut group = Vec::new();
                    group.append(&excess_cards);
                    if idx == 0{
                        for card in high_card - 4..high_card + 1{
                            group.push(card);
                        }
                    }
                    potential_groups.insert(idx, group);
                }
            }

        }


    }
    res
}

fn enumerate_all_groups<T>(vec: &[T], group_size: u8) -> Vec<Vec<T>>{
    let res = Vec::new();

}
mod deck;
use deck::Deck;
use deck::Card;

#[derive(Debug)]
struct Player<'a> {
    hand: Vec<Box<Card<'a>>>,
    deck: Deck<'a>,
    graveyard: Vec<Box<Card<'a>>>
}

impl<'a> Player<'a> {
    fn new(deck_size: u8) -> Self {
        return Self {
            hand: Vec::new(),
            deck: Deck::new(deck_size),
            graveyard: Vec::new(),
        }
    }
}

fn main() {
    let mut me = Player::new(60);


    println!("{me:#?}")
}

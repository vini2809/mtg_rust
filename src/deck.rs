use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct Card<'a> {
    id: u32,
    img: &'a str,
}


impl<'a> Card<'a> {
    pub fn new(id: u32, img_link: &str) -> Self {
        return Self {
            id,
            img: ""
        }
    }
}

#[derive(Debug)]
pub struct Deck<'a> {
    cards: Vec<Box<Card<'a>>>,
    size: u8,
}

impl<'a> Deck<'a> {
    pub fn new(size: u8) -> Deck<'a> {
        return Self {
            cards: Vec::<Box<Card>>::with_capacity(size as usize),
            size,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn draw(&mut self) -> Option<Box<Card>> {
        if self.size > 0 {
            self.size -= 1;
        }
        
        return self.cards.pop();
        
    }

    pub fn put_top(&mut self, card: Box<Card<'a>>) {
        self.cards.push(card);
        self.size += 1;
    } 

    pub fn put_bottom(&mut self, card: Box<Card<'a>>) {
        self.cards.insert(0, card);
        self.size += 1;
    }

    pub fn put_in_xth(&mut self, card: Box<Card>) {

    }

    pub fn scry(&mut self, x: u8) -> Option<&[Box<Card>]> {
        if self.size == 0 || x == 0 {
            return None;
        }

        if x >= self.size {
            return self.cards.get(..);
        }

        return self.cards.get((self.size - x) as usize..self.size as usize);
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}
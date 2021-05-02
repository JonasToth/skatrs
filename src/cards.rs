use rand::seq::SliceRandom;
use rand::thread_rng;
use std::char;
use std::convert::TryInto;
use std::fmt;
use std::vec::Vec;

#[derive(Copy, Clone)]
pub enum Pack {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
}

#[derive(Copy, Clone)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Copy, Clone)]
pub struct Card {
    card: Pack,
    color: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol: u32 = match self.color {
            Suit::Clubs => 0x2663,
            Suit::Spades => 0x2660,
            Suit::Hearts => 0x2661,
            Suit::Diamonds => 0x2662,
        };

        let card = match self.card {
            Pack::Ace => "A",
            Pack::King => "K",
            Pack::Queen => "Q",
            Pack::Jack => "J",
            Pack::Ten => "10",
            Pack::Nine => "9",
            Pack::Eight => "8",
            Pack::Seven => "7",
        };

        write!(f, "{}{:2} ", char::from_u32(symbol).unwrap(), card)
    }
}

#[derive(Clone)]
pub struct Deck {
    cards: [Card; 32],
}

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.cards.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Skat {
    cards: [Card; 2],
}
impl fmt::Display for Skat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.cards.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: [
                Card {
                    card: Pack::Ace,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::King,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::Queen,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::Jack,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::Ten,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::Nine,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::Eight,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::Seven,
                    color: Suit::Clubs,
                },
                Card {
                    card: Pack::Ace,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::King,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::Queen,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::Jack,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::Ten,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::Nine,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::Eight,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::Seven,
                    color: Suit::Spades,
                },
                Card {
                    card: Pack::Ace,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::King,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::Queen,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::Jack,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::Ten,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::Nine,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::Eight,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::Seven,
                    color: Suit::Hearts,
                },
                Card {
                    card: Pack::Ace,
                    color: Suit::Diamonds,
                },
                Card {
                    card: Pack::King,
                    color: Suit::Diamonds,
                },
                Card {
                    card: Pack::Queen,
                    color: Suit::Diamonds,
                },
                Card {
                    card: Pack::Jack,
                    color: Suit::Diamonds,
                },
                Card {
                    card: Pack::Ten,
                    color: Suit::Diamonds,
                },
                Card {
                    card: Pack::Nine,
                    color: Suit::Diamonds,
                },
                Card {
                    card: Pack::Eight,
                    color: Suit::Diamonds,
                },
                Card {
                    card: Pack::Seven,
                    color: Suit::Diamonds,
                },
            ],
        }
    }

    pub fn randomize(&mut self) -> &mut Deck {
        let mut rng = thread_rng();
        for _ in 1..20 {
            self.cards.shuffle(&mut rng);
        }

        self
    }

    pub fn deal(&mut self) -> (Hand, Hand, Hand, Skat) {
        let p1 = Hand {
            cards: self.cards[0..10]
                .try_into()
                .expect("Splitting deck into hands failed"),
        };
        let p2 = Hand {
            cards: self.cards[10..20]
                .try_into()
                .expect("Splitting deck into hands failed"),
        };
        let p3 = Hand {
            cards: self.cards[20..30]
                .try_into()
                .expect("Splitting deck into hands failed"),
        };
        let skat = Skat {
            cards: self.cards[30..32]
                .try_into()
                .expect("Splitting deck into hands failed"),
        };
        (p1, p2, p3, skat)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }

    println!("{:?}", deck);
}

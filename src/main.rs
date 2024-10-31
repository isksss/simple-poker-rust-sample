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
    let suit = Suit::Spade;
    let rank = 1;
    let card = Card { suit, rank };
    println!("{:?}", card);
}

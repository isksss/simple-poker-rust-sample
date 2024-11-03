use rand::seq::SliceRandom;

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

    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    // println!("{:?}", deck);

    // 手札
    let mut hand: Vec<Card> = Vec::new();
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    // TODO: ここの意味がよくわからない
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札を表示
    println!("--- 手札 ---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {}", i + 1, card.suit, card.rank);
    }

    // 入れ替えるカードを選択
    println!("入れ変えるカードを選択してください。[1-5]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    // 入れ替えるカードを交換
    for i in input {
        hand[i - 1] = deck.pop().unwrap();
    }

    // 手札を表示
    println!("--- 手札 ---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {}", i + 1, card.suit, card.rank);
    }
}

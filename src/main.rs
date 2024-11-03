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

    // 役を判定

    // フラッシュ
    let is_flush = hand.iter().all(|card| card.suit == hand[0].suit);
    // ストレート
    let is_straight = hand.windows(2).all(|w| w[0].rank + 1 == w[1].rank);
    // フォーカード
    let is_fourcard = hand.windows(4).any(|w| w[0].rank == w[3].rank);
    // スリーカード
    let is_threecard = hand.windows(3).any(|w| w[0].rank == w[2].rank);
    // ツーペア
    let is_twopair = hand.windows(2).filter(|w| w[0].rank == w[1].rank).count() == 2;
    // ワンペア
    let is_onepair = hand.windows(2).any(|w| w[0].rank == w[1].rank);

    if is_flush && is_straight {
        println!("ストレートフラッシュ");
    } else if is_fourcard {
        println!("フォーカード");
    } else if is_threecard && is_onepair {
        println!("フルハウス");
    } else if is_flush {
        println!("フラッシュ");
    } else if is_straight {
        println!("ストレート");
    } else if is_threecard {
        println!("スリーカード");
    } else if is_twopair {
        println!("ツーペア");
    } else if is_onepair {
        println!("ワンペア");
    } else {
        println!("ハイカード");
    }
}

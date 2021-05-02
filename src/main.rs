mod cards;

use cards::Deck;

fn main() {
    let (player1, player2, player3, skat) = Deck::new().randomize().deal();

    println!("P1: {}", player1);
    println!("P2: {}", player2);
    println!("P3: {}", player3);
    println!("Sk: {}", skat);
}

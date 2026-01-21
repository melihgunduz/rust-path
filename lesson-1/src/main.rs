
use rand::{rng, seq::SliceRandom};



#[derive(Debug)] 
// trait: a set of functions
// derive: specifies which 'traits' to automatically implement for this struct, called 'derive attribute'
// Debug: trait name
// whole statement: defines 'attributes' for the Duck struct, 'Hey, compiler, automatically add all the 'Debug' functions to this struct
struct Deck {
    cards: Vec<String>,
}

// inherent implementations are fancy term of 'add a function to a struct'
// used to define methods and associated functions
// associated function: tied to the struct definition (use when you have functionality not tied to a specific instance)
// method: operates on a specific instance of a sturct (use when you need to read or change fields on a specific instance)
impl Deck {
    fn new() -> Self {
    // Vector's are dynamic sized (can grown/shrink), Array's are fixed sized (can't grow/shrink)

    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    // Bindings are immutable (can't change) by default.

    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    // If you delete semicolon from end of the line, rust returns the value automatically, this named as 'implicit return'
    Deck {cards}    
    }

    fn shuffle (&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    // need some error handling
    fn deal(&mut self, num_cards:usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}


fn main() {
    let mut deck = Deck::new();

    println!("{}", deck.cards.len());

    // need some error handling
    deck.shuffle();
    let cards = deck.deal(3);
    let cards_1 = deck.deal(3);
    // let cards_2 = deck.deal(3);

    let missing_player:u8 = 1;

    println!("Here's your hand: {:#?}", cards);
    println!("Here's player 1 hand: {:#?}", cards_1);
    // println!("Here's player 2 hand: {:#?}", cards_2);

    if deck.cards.len() == 0 {
        println!("All cards have dealed");
    } else { println!("{} Player missing", missing_player)};
    
}

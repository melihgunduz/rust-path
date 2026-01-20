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
}


fn main() {
    let deck = Deck::new();

    println!("Here's your deck: {:#?}", deck.cards);
    
}

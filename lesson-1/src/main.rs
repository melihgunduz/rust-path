#[derive(Debug)] 
// trait: a set of functions
// derive: specifies which 'traits' to automatically implement for this struct, called 'derive attribute'
// Debug: trait name
// whole statement: defines 'attributes' for the Duck struct, 'Hey, compiler, automatically add all the 'Debug' functions to this struct
struct Deck {
    cards: Vec<String>,
}




fn main() {
    // List of 'suits' - 'hearts', 'spades', 'diamonds'
    // List of 'values' - 'ace', 'two', 'three'

    // Double nested for loop

    // Vector's are dynamic sized (can grown/shrink), Array's are fixed sized (can't grow/shrink)

    let suits = vec!["Hearts", "Spades", "Diamonds"];
    let values = vec!["Ace", "Two", "Three"];

    let deck = Deck {cards: vec![]};


    println!("Here's your deck: {:?}", deck);
}

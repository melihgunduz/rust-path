use std::thread::Thread;

#[derive(Debug)]

enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Podcast(u32),
    Placeholder
}


impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Book: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Book: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
            
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media:Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            MightHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            MightHaveAValue::NoValueAvailable
        }
    }
}


enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable
}




fn main () {
    let audiobook = Media::Audiobook { 
        title: String::from("An audiobook") 
    };

    let good_movie = Media::Movie { 
        title: String::from("Good movie"), 
        director: String::from("Perfect director") 
    };

    let bad_book = Media::Book { 
        title: String::from("A low scored book"), 
        author: String::from("Regular author")
    };
    
    let podcast = Media::Podcast(10);

    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // get function belongs to every vector
    // Rust doesn't have null, nil or undefined
    // If you want to work with Option you have to use pattern matching
    // (the 'if let' thing) or a match statement
    // Has two variants - 'Some(value)' and 'None'
    // Forces you to handle the case in which you have a value
    // and the case in which you don't
    
    /* match catalog.items.get(0) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index")
        }
    } */

    // let item = catalog.get_by_index(0);
    // println!("{:#?}", item);

    // match catalog.get_by_index(35) {
    //     MightHaveAValue::ThereIsAValue(value) => 
    //     {
    //         println!("Item {:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("No value at this index!")
    //     }
    // }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(220) {
        println!("Item in pattern match: {:#?}", value)
    } else {
        println!("No value")
    }
}
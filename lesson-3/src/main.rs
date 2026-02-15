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

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
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


    // unwrap(), If item is a 'Some' returns the value in the Some, If item is None, panics! Use for quick debugging or examples
    // expect("there should be a value here"), same with unwrap but prints provided debug message if item is a 'None' and panics! Use when we want to crash if there is no value
    // unwrap_or(&placeholder), If item is 'Some' returns the value in the Some else returns the provided default value. Use when it makes sense to provide a fallback value
    let item = catalog.get_by_index(0);
    let placeholder = Media::Placeholder;
    println!("{:#?}",item.unwrap_or(&placeholder));

    // match catalog.get_by_index(3) {
    //     Some(value) => 
    //     {
    //         println!("Item {:#?}", value);
    //     }
    //     None => {
    //         println!("No value at this index!")
    //     }
    // }

    // if let Some(value) = catalog.get_by_index(2) {
    //     println!("Item in pattern match: {:#?}", value)
    // } else {
    //     println!("No value")
    // }
}
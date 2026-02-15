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
}

fn print_media (media: Media) {
    println!("{:#?}", media);
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

    println!("{:#?}", catalog);
}
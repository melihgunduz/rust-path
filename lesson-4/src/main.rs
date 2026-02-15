mod content;


use content::catalog::Catalog;
use content::media::Media;

fn main() {
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

    let item = catalog.get_by_index(0);
    let placeholder = Media::Placeholder;
    println!("{:#?}",item.unwrap_or(&placeholder));
}

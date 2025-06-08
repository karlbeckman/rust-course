mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let book = Media::Book { title: String::from("Normal People"), author: String::from("Sally Rooney") };
    let movie = Media::Movie { title: String::from("Dark Knight Rises"), director: String::from("Christopher Nolan") };
    let audio_book = Media::Audiobook { title: String::from("Shogun") };
    let podcast = Media::Podcast(121);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audio_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.get_by_index(0) {
        Some(item) => {
            println!("{:#?}", item);
        }
        None => println!("Index out of bounds.")
    }

}

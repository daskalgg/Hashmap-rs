use hashmap::HashMap;
fn main() {
    let mut book_reviews = HashMap::<String, String>::new();
    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    book_reviews.remove("The Adventures of Sherlock Holmes".to_string());

    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book.to_string()) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed.")
        }
    }

    for (book, review) in &book_reviews {
        println!("{book}: \"{review}\"");
    }

    if !book_reviews.contains_key("Les Misérables".to_string()) {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                 book_reviews.len());
    }
}

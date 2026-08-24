use std::io;

fn main() {
    // Give the program a sentence or paragraph and have it report:
    // Characters: 142
    // Words: 27
    // Longest word: "something"

    println!("Hello, welcome to the text analyser!");
    println!(
        "This program analyses yourr sentences to provide charcter count, number of words and the longest word in your sentence"
    );

    let mut sentence = String::new();
    println!("Kindly provide your sentence: ");
    // Collect user's input
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read sentence");

    let number_of_characters = find_number_of_characters(&sentence);
    let number_of_words = find_number_of_words(&sentence);
    println!("Characters: {}", number_of_characters);
    println!("Words: {}", number_of_words);
}

// Number of characters
fn find_number_of_characters(sentence: &String) -> usize {
    sentence
        .chars()
        .filter(|character| !character.is_whitespace())
        .count()
}

// Number of words
fn find_number_of_words(sentence: &String) -> usize {
    sentence.split_whitespace().count()
}

// Longest word

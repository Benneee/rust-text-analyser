use std::io;

fn main() {
    // Give the program a sentence or paragraph and have it report:
    // Characters: 142
    // Words: 27
    // Longest word: "something"

    println!("Hello, welcome to the text analyser!");
    println!(
        "This program analyses your sentences to provide charcter count, number of words and the longest word in your sentence"
    );

    let mut sentence = String::new();
    println!("Kindly provide your sentence: ");
    // Collect user's input
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read sentence");

    let number_of_characters = find_number_of_characters(&sentence);
    let number_of_words = find_number_of_words(&sentence);
    let longest_word = find_longest_word(&sentence);
    println!("Characters: {}", number_of_characters);
    println!("Words: {}", number_of_words);
    println!("Longest word: {}", longest_word);
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
fn find_longest_word(sentence: &String) -> &str {
    // Collect each word with its count in an object
    // let mut word_count_pair: HashMap<&str, usize> = HashMap::new();

    // // .split_whitespace here returns a Vec<_>
    // for word in sentence.split_whitespace() {
    //     word_count_pair.insert(word, word.len());
    // }

    // // max_by_key allows us access to both the key and the value
    // // using .values().max().unwrap() would have returned just the count of the longest word
    // match word_count_pair.iter().max_by_key(|&(_word, length)| length) {
    //     Some((word, _)) => {
    //         println!("Longest word: {}", word);
    //     }
    //     None => {
    //         println!("The sentence contained no words.");
    //     }
    // }

    // The above approach preserves everything including text that is not needed
    // We can do a single pass keeping only the current longest word and discarding what's not needed
    // Need to save the first word in the sentence first
    let mut current_longest = sentence.split_whitespace().next().unwrap();
    // Go over each word, discard the one that is not greater than the current longest
    for word in sentence.split_whitespace() {
        if word.len() > current_longest.len() {
            current_longest = word;
        }
    }
    current_longest
}

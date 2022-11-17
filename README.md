# Rust-Tokenizer

Sentence tokenizer written in RUST. The goal of the project is to develop a clone of keras tokenizer while familiarizing with RUST language.

## Functionalities
Given a corpus in train.txt it will train a basic tokenizer that will store each word as an integer. After that it will use that tokenizer to decode a "This is a test." sentence.

![demo](https://raw.githubusercontent.com/Polifack/Rust-Tokenizer/main/pics/demo.png)

## Roadmap
- Encode words as integers: Done
- Add padding capabilities: Done
- Turn into a class and turn constants (filter_chars, oov_char, turn_lower, split_char, num_words) into parameters.
- Add option to tokenize into character level. 
- Add n-grams tokenizer.

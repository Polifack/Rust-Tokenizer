use regex::Regex;
use std::collections::HashMap;

fn preprocess(input: String) -> String {
    // Use regexp to remove all duplicate whitespace
    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(&input, " ");

    // Use regexp to remove all tabs
    let re = Regex::new(r"\t").unwrap();
    let result = re.replace_all(&result, "");

    // Use regexp to remove all newlines
    let re = Regex::new(r"\n").unwrap();
    let result = re.replace_all(&result, "");
    
    let result = result.replace(",", " ,");
    let result = result.replace(".", " .");
    let result = result.replace("?", " ?");
    let result = result.replace("!", " !");
    let result = result.replace("(", " (");
    let result = result.replace(")", " )");
    let result = result.replace(":", " :");
    let result = result.replace(";", " ;");

    return result.to_string();
}

fn split_sentence(input: String) -> Vec<String> {
    // Use regexp to split on punctuation
    let result: Vec<_> = input.split_inclusive(&[',',' ','.',';'][..]).collect();
    let result_string: Vec<String> = result.iter().map(|s| s.to_string()).collect();
    let result_trimmed: Vec<String> = result_string.iter().map(|s| s.trim().to_string()).collect();
    return result_trimmed;
}

fn create_word_vocabulary(input: Vec<String>) -> HashMap<String, i32> {
    
    let mut word_vocabulary: HashMap<String, i32> = HashMap::new();
    let mut word_number = 0;
    
    for i in input {
        word_vocabulary.insert(i, word_number);
        word_number +=1;
    }
    return word_vocabulary;
}

fn tokenize_sentence(input: Vec<String>, word_vocabulary: HashMap<String, i32>, oov: String) -> Vec<i32> {
    let mut result = Vec::new();
    println!("OOV: {}", oov);
    for i in input {
        // Try get the word from the vocabulary
        println!("Word: {}", i);
        println!("Word number: {:?}", word_vocabulary.get(&i));
        let word_number = match word_vocabulary.get(&i){
            Some(w) => w,
            None => word_vocabulary.get(&oov).unwrap(),
        };
        result.push(*word_number);
    }
    return result;
}

fn main(){
    // define oov
    let oov = "<OOV>".to_string();

    // read filepath
    let filepath: String = "./sentence.txt".to_string();

    // read file
    let contents = std::fs::read_to_string(filepath.trim()).expect("Failed to read file");

    // split file into lines
    let lines: Vec<&str> = contents.split_inclusive(".").collect();

    // Iterate over lines
    let mut sentences_set: Vec<String> = Vec::new();
    sentences_set.push(oov.clone());
    let mut words_set: Vec<String> = Vec::new();
    words_set.push(oov.clone());
    
    for line in lines{
        // preprocess line to remove extra whitespaces or tabs
        let preprocess_line = preprocess(line.to_string());
        let trim_sentence   = preprocess_line.trim();
        // store line in sentences
        sentences_set.push(trim_sentence.to_string());
        // Iterate over words
        let words: Vec<String> = split_sentence(preprocess_line);
        for word in words{
            // store word in words
            if !words_set.contains(&word) && (&word != ""){
                words_set.push(word);
            }
        }

    }

    println!("Number of sentences: {}", sentences_set.len());
    println!("Number of words: {}", words_set.len());

    let word_vocabulary = create_word_vocabulary(words_set);
    let test = tokenize_sentence(split_sentence(preprocess("This is a test.".to_string())), word_vocabulary, oov);
    println!("Tokenized sentence: {:?}", test);
}
fn most_frequent_word(text: &str) -> (String, usize) {
    let mut words: Vec<&str> = Vec::new(); 
    let mut counts: Vec<usize> = Vec::new(); 

    for word in text.split_whitespace() {
        
        if let Some(index) = words.iter().position(|&w| w == word) {
            counts[index] += 1; 
        } else {
            words.push(word); 
            counts.push(1);  
        }
    }

    let mut max_word = "";
    let mut max_count = 0;

    for (i, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_word = words[i];
            max_count = count;
        }
    }

    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

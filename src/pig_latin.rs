// Convert strings to pig latin. The first consonant of each word is moved to the
// end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that
// start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”). Keep in mind the details about UTF-8 encoding!
pub fn pig_latin(sentence: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    sentence
        .split_whitespace()
        .map(|word| {
            let v_index = word
                .char_indices()
                .find(|(_, c)| vowels.contains(&c))
                .unwrap()
                .0;
            let mut latin = String::with_capacity(word.len() + 3);
            latin.push_str(&word[v_index..]);
            if v_index == 0 {
                latin.push_str("hay");
            } else {
                latin.push_str(&word[..v_index]);
                latin.push_str("ay");
            }
            latin
        })
        .collect::<Vec<String>>()
        .join(" ")
}

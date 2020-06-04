fn pig_latinise(word: &String) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    let first_letter = &word[0..1];
    if vowels.contains(&first_letter) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", &word[1..], first_letter)
    }
}

#[cfg(test)]
mod tests {
    use crate::exercises::pig_latin::pig_latinise;

    #[test]
    fn test_pignlatinise_word_starting_with_consonant() {
        assert_eq!(pig_latinise(&String::from("first")), String::from("irst-fay"));
    }

    #[test]
    fn test_pignlatinise_word_starting_with_vowel() {
        assert_eq!(pig_latinise(&String::from("apple")), String::from("apple-hay"));
    }
}

pub mod demo {
    use crate::exercises::pig_latin::pig_latinise;

    fn strings_from(words: Vec<&str>) -> Vec<String> {
        words.iter().map(|w| String::from(*w)).collect()
    }

    fn show(english: &String, pig_latin: &String) {
        println!("\"{}\" -> \"{}\"", english, pig_latin);
    }

    fn demo_words() {
        info!("Showing words");
        for word in strings_from(vec!["first", "apple"]) {
            show(&word, &pig_latinise(&word));
        }
    }

    fn format_greeting(words: Vec<String>) -> String {
        let concatenated = words.join(" ");
        format!("{}{}!", &concatenated[0..1].to_string().to_uppercase(), &concatenated[1..])
    }

    fn demo_greeting_phrase() {
        info!("Showing phrases");
        let english_words = strings_from(vec!["hello", "world"]);
        let piglatin_words: Vec<String> = english_words.iter().map(pig_latinise).collect();
        show(&format_greeting(english_words), &format_greeting(piglatin_words));
    }

    pub fn demo_pig_latin() {
        info!("Showing Pig Latin examples");
        demo_words();
        demo_greeting_phrase();
    }
}

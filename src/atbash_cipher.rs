// Atbash is a monoalphabetic substitution cipher originally used to encrypt the Hebrew
// alphabet. It is a very simple cipher wherein the alphabet used is mapped to its reverse
// so the first letter becomes the last, the second letter becomes the second last letter,
// and so forth.

// Plain: abcdefghijklmnopqrstuvwxyz
// Atbash Cipher: zyxwvutsrqponmlkjihgfedcba

// Since there is only one way to perform this, the Atbash cipher provides no communications
// security due to the lack of the key. If multiple collating orders are available and one was
// encrypted as a key, this would provide only marginal security improvements as only a few 
// letters can give away which one was used.

// Traditionally, the ciphertext is written out in groups of fixed length (5 letters) with
// puncuation excluded making it harder to guess things based on word boundaries. This is why 
// we call .chunks(5) within atbash()

use std::collections::HashMap;

const FORWARDS: &'static str = "abcdefghijklmnopqrstuvwxyz";
const BACKWARDS: &'static str = "zyxwvutsrqponmlkjihgfedcba";

fn atbash(text: &str) -> Vec<String> {
    // Mapping the substitution using .zip()
    let substitution: HashMap<char, char> = FORWARDS.chars().zip(BACKWARDS.chars()).collect();

    text.to_lowercase()
        .chars()
        .filter(|character| character.is_digit(36))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().map(|&k| if substitution.contains_key(&k) { substitution.get(&k).unwrap().clone() } else { k }))
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<String>>()
}

fn transcode(text: &str) -> String {
    atbash(text).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atbash() {

        let mut test1 = "test";
        assert_eq!(transcode(&mut test1), "gvhg");

        let mut test2 = "gvhg";
        assert_eq!(transcode(&mut test2), "test");

        let mut test3 = "Hello there";
        assert_eq!(transcode(&mut test3), "svoolgsviv");

        let mut test4 = "General Kenobi";
        assert_eq!(transcode(&mut test4), "tvmvizopvmlyr");

        let mut test5 = "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt";
        assert_eq!(transcode(&mut test5), "thequickbrownfoxjumpsoverthelazydog");

    }
}
// The Caesar Cipher, also known as Caesar's code or simply the shift cipher, is one of the earliets and simplest methods
// of encryption. It is a type of substitution cipher wherein each letter in plaintext is replaced by a letter some fixed 
// number of positions down the alphabet. The Caesar Cipher is named after Julius Caesar, who, according to Suetonis, used
// it with a shift of three (A becomes D when encrypting, and D becomes I when decrypting) to protect messages of military
// significance

// Plain: A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
// Caesar Cipher: D E F G H I J K L M N O P Q R S T U V W X Y Z A B C

// The Caesar Cipher can be broken even in a ciphertext-only scenario since there are only a limited number of possible shifts
// (25 in English). An attacker can mount a brute force attack by deciphering the message, or part of it, using each possible 
// shift. It can also be easily cracked when graphing  the frequency distribution of the letters in cipher text and by knowing
// the expected distribution of those letters in the original language of the plaintext. 

// Encrypting a text multiple times with the Caesar Cipher provides no additional security because, say, two encryptions shift A 
// and shift B will be equivalent to a single encryption of A + B. The set of encryption operations under each possible key forms
// a group under composition 

fn caesar(message: &str, shift: u8) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first_char = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first_char + (c as u8 + shift - first_char) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar() {

        let mut test1 = "";
        assert_eq!(caesar(&mut test1, 10), "");

        let mut test2 = "ATTACKATONCE";
        assert_eq!(caesar(&mut test2, 4), "EXXEGOEXSRGI");

        let mut test3 = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
        assert_eq!(caesar(&mut test3, 23), "QEB NRFZH YOLTK CLU GRJMP LSBO QEB IXWV ALD");

        let mut test4 = "a";
        assert_eq!(caesar(&mut test4, 3), "d");

        let mut test5 = "general kenobi";
        assert_eq!(caesar(&mut test5, 23), "dbkboxi hbklyf");

    }
}
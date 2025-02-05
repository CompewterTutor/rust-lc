fn reverse_words(s: String) -> String {
    let mut words = s.split_whitespace().collect::<Vec<&str>>();
    words.reverse();
    words.join(" ")
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("the sky is blue".to_string()), "blue is sky the");
        assert_eq!(reverse_words("  hello world  ".to_string()), "world hello");
        assert_eq!(reverse_words("a good   example".to_string()), "example good a");
        assert_eq!(reverse_words("".to_string()), "");
        assert_eq!(reverse_words("single".to_string()), "single");
    }
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut count = 0;
    let mut last = 0;
    for c in s.chars() {
        if c == ' ' {
            if count != 0 {
                last = count;
            }
            count = 0;
        } else {
            count += 1;
        }
    }
    if count != 0 {
        last = count;
    }
    last
}

pub fn slow_length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().map_or(0, |word| word.len() as i32)
}

fn main() {
    println!("Length of Last Word");
}

mod tests {
    use super::*;

    #[cfg(test)]
    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
        assert_eq!(length_of_last_word("".to_string()), 0);
        assert_eq!(length_of_last_word("a".to_string()), 1);
        assert_eq!(length_of_last_word("a ".to_string()), 1);
        assert_eq!(length_of_last_word("day".to_string()), 3);
        assert_eq!(length_of_last_word("day ".to_string()), 3);
        assert_eq!(length_of_last_word(" day".to_string()), 3);
        assert_eq!(length_of_last_word(" day ".to_string()), 3);
    }

    #[cfg(test)]
    #[test]
    fn test_slow_length_of_last_word() {
        assert_eq!(slow_length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(slow_length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(slow_length_of_last_word("luffy is still joyboy".to_string()), 6);
        assert_eq!(slow_length_of_last_word("".to_string()), 0);
        assert_eq!(slow_length_of_last_word("a".to_string()), 1);
        assert_eq!(slow_length_of_last_word("a ".to_string()), 1);
        assert_eq!(slow_length_of_last_word("day".to_string()), 3);
        assert_eq!(slow_length_of_last_word("day ".to_string()), 3);
        assert_eq!(slow_length_of_last_word(" day".to_string()), 3);
        assert_eq!(slow_length_of_last_word(" day ".to_string()), 3);
    }
}
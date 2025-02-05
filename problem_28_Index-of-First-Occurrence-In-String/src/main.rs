pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
        assert_eq!(str_str("".to_string(), "".to_string()), 0);
        assert_eq!(str_str("a".to_string(), "a".to_string()), 0);
        assert_eq!(str_str("mississippi".to_string(), "issip".to_string()), 4);
    }
}

fn main() {
    println!("Hello, world!");
}

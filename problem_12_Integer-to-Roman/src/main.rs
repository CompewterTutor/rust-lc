pub fn int_to_roman(mut num: i32) -> String {
    let symbols = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
    ];

    let mut result = String::new();
    
    for &(value, symbol) in symbols.iter() {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }

    result
}

fn main() {
    // You can keep the main function empty or use it for other purposes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}

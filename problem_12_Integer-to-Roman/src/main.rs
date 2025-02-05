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
    let num1 = 3749;
    let num2 = 58;
    let num3 = 1994;

    println!("{} -> {}", num1, int_to_roman(num1)); // MMMDCCXLIX
    println!("{} -> {}", num2, int_to_roman(num2)); // LVIII
    println!("{} -> {}", num3, int_to_roman(num3)); // MCMXCIV
}

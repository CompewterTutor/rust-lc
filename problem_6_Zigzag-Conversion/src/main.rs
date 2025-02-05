pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut rows = vec![String::new(); num_rows as usize];
    let mut current_row = 0;
    let mut going_down = false;
    for c in s.chars() {
        rows[current_row as usize].push(c);
        if current_row == 0 || current_row == num_rows - 1 {
            going_down = !going_down;
        }

        if going_down {
            current_row += 1;
        } else {
            current_row -= 1;
        }
    }
    rows.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
        assert_eq!(convert("A".to_string(), 1), "A".to_string());
    }
}

fn main() {
    println!("Hello, world!");
}

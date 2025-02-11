// Given an m x n matrix, return all elements of the matrix in spiral order.

 

// Example 1:

// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [1,2,3,6,9,8,7,4,5]

// Example 2:

// Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
// Output: [1,2,3,4,8,12,11,10,9,5,6,7]

 

// Constraints:

//     m == matrix.length
//     n == matrix[i].length
//     1 <= m, n <= 10
//     -100 <= matrix[i][j] <= 100

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> { 
    if matrix.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();
    let (mut top, mut bottom) = (0, matrix.len() as isize - 1);
    let (mut left, mut right) = (0, matrix[0].len() as isize - 1);

    while top <= bottom && left <= right
    {
        //left to right
        for col in left..=right {
            result.push(matrix[top as usize][col as usize]);
        }
        top += 1;
        //top to bottom
        for row in top..=bottom {
            result.push(matrix[row as usize][right as usize]);
        }
        right -= 1;
        if top <= bottom {
            //right to left
            for col in (left..=right).rev() {
                result.push(matrix[bottom as usize][col as usize]);
            }
            bottom -= 1;
        }
        if left <= right {
            // bottom to top
            for row in (top..=bottom).rev() {
                result.push(matrix[row as usize][left as usize]);
            }
            left += 1;
        }
        
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order_3x3() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(spiral_order(matrix), expected);
    }

    #[test]
    fn test_spiral_order_3x4() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(spiral_order(matrix), expected);
    }

    #[test]
    fn test_spiral_order_empty() {
        let matrix: Vec<Vec<i32>> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(spiral_order(matrix), expected);
    }

    #[test]
    fn test_spiral_order_single_element() {
        let matrix = vec![vec![1]];
        let expected = vec![1];
        assert_eq!(spiral_order(matrix), expected);
    }
}

fn main() {
    println!("Hello, world!");
}

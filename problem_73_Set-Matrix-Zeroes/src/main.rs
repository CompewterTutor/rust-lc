// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.

// You must do it in place.

 

// Example 1:

// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
// Output: [[1,0,1],[0,0,0],[1,0,1]]

// Example 2:

// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]

 

// Constraints:

//     m == matrix.length
//     n == matrix[0].length
//     1 <= m, n <= 200
//     -231 <= matrix[i][j] <= 231 - 1

 

// Follow up:

//     A straightforward solution using O(mn) space is probably a bad idea.
//     A simple improvement uses O(m + n) space, but still not the best solution.
//     Could you devise a constant space solution?



pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut is_first_row_zero = false;
    let mut is_first_col_zero = false;

    //check for first row
    for j in 0..n {
        if matrix[0][j] == 0 {
            is_first_row_zero = true;
            break;
        }
    }

    for i in 0..m {
        if matrix[i][0] == 0 {
            is_first_col_zero = true;
            break;
        }
    }

    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    // Zero out cells based on markers
    for i in 1..m {
        for j in 1..n {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    // Zero out the first row if needed
    if is_first_row_zero {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }

    // Zero out the first column if needed
    if is_first_col_zero {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn test_example_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
    }

    #[test]
    fn test_no_zeroes() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn test_all_zeroes() {
        let mut matrix = vec![vec![0, 0], vec![0, 0]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0], vec![0, 0]]);
    }
}

fn main() {
    println!("Hello, world!");
}

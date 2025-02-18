// Given an array of intervals where intervals[i] = [starti, endi], 
// merge all overlapping intervals, and return an array of the non-overlapping 
// intervals that cover all the intervals in the input.


// Example 1:

// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].

// Example 2:

// Input: intervals = [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.


// Constraints:

//     1 <= intervals.length <= 104
//     intervals[i].length == 2
//     0 <= starti <= endi <= 104

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }
    //sort by start_time
    intervals.sort_by_key(|interval| interval[0]);
    let mut merged:Vec<Vec<i32>> = Vec::new();

    for interval in intervals {
        if let Some(last) = merged.last_mut() {
            if last[1] >= interval[0] {
                //merge overlapping intervals
                last[1] = last[1].max(interval[1]);
            } else {
                merged.push(interval);
            }
        } else {
            merged.push(interval);
        }
    }
    merged
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(merge(intervals), expected);
    }

    #[test]
    fn test_merge_intervals_single_overlap() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        assert_eq!(merge(intervals), expected);
    }

    #[test]
    fn test_merge_intervals_no_overlap() {
        let intervals = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(merge(intervals), expected);
    }

    #[test]
    fn test_merge_intervals_contained() {
        let intervals = vec![vec![1, 10], vec![2, 6], vec![8, 10]];
        let expected = vec![vec![1, 10]];
        assert_eq!(merge(intervals), expected);
    }
}

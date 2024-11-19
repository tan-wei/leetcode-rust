/**
 * [1769] Minimum Number of Operations to Move All Balls to Each Box
 *
 * You have n boxes. You are given a binary string boxes of length n, where boxes[i] is '0' if the i^th box is empty, and '1' if it contains one ball.
 * In one operation, you can move one ball from a box to an adjacent box. Box i is adjacent to box j if abs(i - j) == 1. Note that after doing so, there may be more than one ball in some boxes.
 * Return an array answer of size n, where answer[i] is the minimum number of operations needed to move all the balls to the i^th box.
 * Each answer[i] is calculated considering the initial state of the boxes.
 *  
 * Example 1:
 *
 * Input: boxes = "110"
 * Output: [1,1,3]
 * Explanation: The answer for each box is as follows:
 * 1) First box: you will have to move one ball from the second box to the first box in one operation.
 * 2) Second box: you will have to move one ball from the first box to the second box in one operation.
 * 3) Third box: you will have to move one ball from the first box to the third box in two operations, and move one ball from the second box to the third box in one operation.
 *
 * Example 2:
 *
 * Input: boxes = "001011"
 * Output: [11,8,5,4,3,4]
 *  
 * Constraints:
 *
 * 	n == boxes.length
 * 	1 <= n <= 2000
 * 	boxes[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/solutions/5743499/improved-algorithm-with-o-n-runtime/
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let mut result: Vec<i32> = vec![0; n];
        let mut moves: i32 = 0;
        let mut count: i32 = 0;

        for i in 0..n {
            result[i] += moves;
            if boxes.chars().nth(i) == Some('1') {
                count += 1;
            }
            moves += count;
        }

        moves = 0;
        count = 0;

        for i in (0..n).rev() {
            result[i] += moves;
            if boxes.chars().nth(i) == Some('1') {
                count += 1;
            }
            moves += count;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1769_example_1() {
        let boxes = "110".to_string();

        let result = vec![1, 1, 3];

        assert_eq!(Solution::min_operations(boxes), result);
    }

    #[test]
    fn test_1769_example_2() {
        let boxes = "001011".to_string();

        let result = vec![11, 8, 5, 4, 3, 4];

        assert_eq!(Solution::min_operations(boxes), result);
    }
}

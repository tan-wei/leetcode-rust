/**
 * [1007] Minimum Domino Rotations For Equal Row
 *
 * In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom halves of the i^th domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
 * We may rotate the i^th domino, so that tops[i] and bottoms[i] swap values.
 * Return the minimum number of rotations so that all the values in tops are the same, or all the values in bottoms are the same.
 * If it cannot be done, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/14/domino.png" style="height: 300px; width: 421px;" />
 * Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
 * Output: 2
 * Explanation:
 * The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
 * If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
 *
 * Example 2:
 *
 * Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
 * Output: -1
 * Explanation:
 * In this case, it is not possible to rotate the dominoes to make one row of values equal.
 *
 *  
 * Constraints:
 *
 * 	2 <= tops.length <= 2 * 10^4
 * 	bottoms.length == tops.length
 * 	1 <= tops[i], bottoms[i] <= 6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
// discuss: https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/solutions/901537/rust-solution/
    pub fn min_domino_rotations(top: Vec<i32>, bottom: Vec<i32>) -> i32 {
        'nums: for num in 1..=6 {
            let mut rotations = (0, 0);
            for domino in top.iter().zip(bottom.iter()) {
                match (*domino.0 == num, *domino.1 == num) {
                    (true, true) => {}
                    (true, false) => rotations.0 += 1,
                    (false, true) => rotations.1 += 1,
                    (false, false) => continue 'nums,
                }
            }
            return std::cmp::min(rotations.0, rotations.1);
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1007_example_1() {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        let result = 2;

        assert_eq!(Solution::min_domino_rotations(tops, bottoms), result);
    }

    #[test]
    fn test_1007_example_2() {
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        let result = -1;

        assert_eq!(Solution::min_domino_rotations(tops, bottoms), result);
    }
}

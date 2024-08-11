/**
 * [1640] Check Array Formation Through Concatenation
 *
 * You are given an array of distinct integers arr and an array of integer arrays pieces, where the integers in pieces are distinct. Your goal is to form arr by concatenating the arrays in pieces in any order. However, you are not allowed to reorder the integers in each array pieces[i].
 * Return true if it is possible to form the array arr from pieces. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: arr = [15,88], pieces = [[88],[15]]
 * Output: true
 * Explanation: Concatenate [15] then [88]
 *
 * Example 2:
 *
 * Input: arr = [49,18,16], pieces = [[16,18,49]]
 * Output: false
 * Explanation: Even though the numbers match, we cannot reorder pieces[0].
 *
 * Example 3:
 *
 * Input: arr = [91,4,64,78], pieces = [[78],[4,64],[91]]
 * Output: true
 * Explanation: Concatenate [91] then [4,64] then [78]
 *
 *  
 * Constraints:
 *
 * 	1 <= pieces.length <= arr.length <= 100
 * 	sum(pieces[i].length) == arr.length
 * 	1 <= pieces[i].length <= arr.length
 * 	1 <= arr[i], pieces[i][j] <= 100
 * 	The integers in arr are distinct.
 * 	The integers in pieces are distinct (i.e., If we flatten pieces in a 1D array, all the integers in this array are distinct).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-array-formation-through-concatenation/
// discuss: https://leetcode.com/problems/check-array-formation-through-concatenation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        pieces.iter().all(|p| {
            p.len() > 0
                && p.iter()
                    .partial_cmp(arr.iter().skip_while(|&c| *c != p[0]).take(p.len()))
                    .unwrap()
                    .is_eq()
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1640_example_1() {
        let arr = vec![15, 88];
        let pieces = vec![vec![88], vec![15]];

        let result = true;

        assert_eq!(Solution::can_form_array(arr, pieces), result);
    }

    #[test]
    fn test_1640_example_2() {
        let arr = vec![49, 18, 16];
        let pieces = vec![vec![16, 18, 49]];

        let result = false;

        assert_eq!(Solution::can_form_array(arr, pieces), result);
    }

    #[test]
    fn test_1640_example_3() {
        let arr = vec![91, 4, 64, 78];
        let pieces = vec![vec![78], vec![4, 64], vec![91]];

        let result = true;

        assert_eq!(Solution::can_form_array(arr, pieces), result);
    }
}

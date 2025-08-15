/**
 * [2121] Intervals Between Identical Elements
 *
 * You are given a 0-indexed array of n integers arr.
 * The interval between two elements in arr is defined as the absolute difference between their indices. More formally, the interval between arr[i] and arr[j] is |i - j|.
 * Return an array intervals of length n where intervals[i] is the sum of intervals between arr[i] and each element in arr with the same value as arr[i].
 * Note: |x| is the absolute value of x.
 *  
 * Example 1:
 *
 * Input: arr = [2,1,3,1,2,3,3]
 * Output: [4,2,7,2,4,4,5]
 * Explanation:
 * - Index 0: Another 2 is found at index 4. |0 - 4| = 4
 * - Index 1: Another 1 is found at index 3. |1 - 3| = 2
 * - Index 2: Two more 3s are found at indices 5 and 6. |2 - 5| + |2 - 6| = 7
 * - Index 3: Another 1 is found at index 1. |3 - 1| = 2
 * - Index 4: Another 2 is found at index 0. |4 - 0| = 4
 * - Index 5: Two more 3s are found at indices 2 and 6. |5 - 2| + |5 - 6| = 4
 * - Index 6: Two more 3s are found at indices 2 and 5. |6 - 2| + |6 - 5| = 5
 *
 * Example 2:
 *
 * Input: arr = [10,5,10,10]
 * Output: [5,0,3,4]
 * Explanation:
 * - Index 0: Two more 10s are found at indices 2 and 3. |0 - 2| + |0 - 3| = 5
 * - Index 1: There is only one 5 in the array, so its sum of intervals to identical elements is 0.
 * - Index 2: Two more 10s are found at indices 0 and 3. |2 - 0| + |2 - 3| = 3
 * - Index 3: Two more 10s are found at indices 0 and 2. |3 - 0| + |3 - 2| = 4
 *
 *  
 * Constraints:
 *
 * 	n == arr.length
 * 	1 <= n <= 10^5
 * 	1 <= arr[i] <= 10^5
 *
 *  
 * Note: This question is the same as <a href="https://leetcode.com/problems/sum-of-distances/description/" target="_blank"> 2615: Sum of Distances.</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/intervals-between-identical-elements/
// discuss: https://leetcode.com/problems/intervals-between-identical-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2121_example_1() {
        let arr = vec![2, 1, 3, 1, 2, 3, 3];

        let result = vec![4, 2, 7, 2, 4, 4, 5];

        assert_eq!(Solution::get_distances(arr), result);
    }

    #[test]
    #[ignore]
    fn test_2121_example_2() {
        let arr = vec![10, 5, 10, 10];

        let result = vec![5, 0, 3, 4];

        assert_eq!(Solution::get_distances(arr), result);
    }
}

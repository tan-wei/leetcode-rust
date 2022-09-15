/**
 * [0823] Binary Trees With Factors
 *
 * Given an array of unique integers, arr, where each integer arr[i] is strictly greater than 1.
 * We make a binary tree using these integers, and each number may be used for any number of times. Each non-leaf node's value should be equal to the product of the values of its children.
 * Return the number of binary trees we can make. The answer may be too large so return the answer modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: arr = [2,4]
 * Output: 3
 * Explanation: We can make these trees: [2], [4], [4, 2, 2]
 * Example 2:
 *
 * Input: arr = [2,4,5,10]
 * Output: 7
 * Explanation: We can make these trees: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 1000
 * 	2 <= arr[i] <= 10^9
 * 	All the values of arr are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-trees-with-factors/
// discuss: https://leetcode.com/problems/binary-trees-with-factors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/binary-trees-with-factors/discuss/1107610/Rust-HashMap-solution
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut hm = arr
            .iter()
            .map(|&n| (n, 1))
            .collect::<std::collections::HashMap<_, _>>();
        let mut arr = arr;
        arr.sort_unstable();
        for i in 1..arr.len() {
            for j in 0..i {
                if arr[i] % arr[j] == 0 {
                    if let Some(&v) = hm.get(&(arr[i] / arr[j])) {
                        let vj = *hm.get_mut(&arr[j]).unwrap();
                        if let Some(vi) = hm.get_mut(&arr[i]) {
                            *vi = (*vi + vj * v) % MOD
                        }
                    }
                }
            }
        }
        (hm.values().sum::<i64>() % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0823_example_1() {
        let arr = vec![2, 4];
        let result = 3;

        assert_eq!(Solution::num_factored_binary_trees(arr), result);
    }

    #[test]
    fn test_0823_example_2() {
        let arr = vec![2, 4, 5, 10];
        let result = 7;

        assert_eq!(Solution::num_factored_binary_trees(arr), result);
    }
}

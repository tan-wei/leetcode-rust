/**
 * [60] Permutation Sequence
 *
 * The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
 * By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
 * <ol>
 * 	"123"
 * 	"132"
 * 	"213"
 * 	"231"
 * 	"312"
 * 	"321"
 * </ol>
 * Given n and k, return the k^th permutation sequence.
 *  
 * Example 1:
 * Input: n = 3, k = 3
 * Output: "213"
 * Example 2:
 * Input: n = 4, k = 9
 * Output: "2314"
 * Example 3:
 * Input: n = 3, k = 1
 * Output: "123"
 *  
 * Constraints:
 *
 * 	1 <= n <= 9
 * 	1 <= k <= n!
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-sequence/
// discuss: https://leetcode.com/problems/permutation-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/permutation-sequence/discuss/585823/Rust-solution%3A-no-dfs-only-math-calculation-0ms2.1MB-both-100
    pub fn get_permutation(n: i32, k: i32) -> String {
        // FACTORIAL of numbers from 0! to 9!
        const FACTORIAL: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
        let mut numbers = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut sequence = String::new();
        // necessary
        let mut k = k - 1;

        for i in 0..n {
            let idx = (n - 1 - i) as usize;
            let div = k / FACTORIAL[idx];

            k = k % FACTORIAL[idx];
            sequence.push(numbers.remove(div as usize));
        }

        sequence
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0060_example_1() {
        let n = 3;
        let k = 3;
        let result = "213".to_string();

        assert_eq!(Solution::get_permutation(n, k), result);
    }

    #[test]
    fn test_0060_example_2() {
        let n = 4;
        let k = 9;
        let result = "2314".to_string();

        assert_eq!(Solution::get_permutation(n, k), result);
    }

    #[test]
    fn test_0060_example_3() {
        let n = 3;
        let k = 1;
        let result = "123".to_string();

        assert_eq!(Solution::get_permutation(n, k), result);
    }
}

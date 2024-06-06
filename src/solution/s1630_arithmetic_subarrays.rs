/**
 * [1630] Arithmetic Subarrays
 *
 * A sequence of numbers is called arithmetic if it consists of at least two elements, and the difference between every two consecutive elements is the same. More formally, a sequence s is arithmetic if and only if s[i+1] - s[i] == s[1] - s[0] for all valid i.
 * For example, these are arithmetic sequences:
 *
 * 1, 3, 5, 7, 9
 * 7, 7, 7, 7
 * 3, -1, -5, -9
 * The following sequence is not arithmetic:
 *
 * 1, 1, 2, 5, 7
 * You are given an array of n integers, nums, and two arrays of m integers each, l and r, representing the m range queries, where the i^th query is the range [l[i], r[i]]. All the arrays are 0-indexed.
 * Return a list of boolean elements answer, where answer[i] is true if the subarray nums[l[i]], nums[l[i]+1], ... , nums[r[i]] can be rearranged to form an arithmetic sequence, and false otherwise.
 *  
 * Example 1:
 *
 * Input: nums = [4,6,5,9,3,7], l = [0,0,2], r = [2,3,5]
 * Output: [true,false,true]
 * Explanation:
 * In the 0^th query, the subarray is [4,6,5]. This can be rearranged as [6,5,4], which is an arithmetic sequence.
 * In the 1^st query, the subarray is [4,6,5,9]. This cannot be rearranged as an arithmetic sequence.
 * In the 2^nd query, the subarray is [5,9,3,7]. This can be rearranged as [3,5,7,9], which is an arithmetic sequence.
 * Example 2:
 *
 * Input: nums = [-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10], l = [0,1,6,4,8,7], r = [4,4,9,7,9,10]
 * Output: [false,true,false,false,true,true]
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	m == l.length
 * 	m == r.length
 * 	2 <= n <= 500
 * 	1 <= m <= 500
 * 	0 <= l[i] < r[i] < n
 * 	-10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/arithmetic-subarrays/
// discuss: https://leetcode.com/problems/arithmetic-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.into_iter()
            .zip(r.into_iter())
            .map(|(li, ri)| {
                let low = *nums[li as usize..=ri as usize].iter().min().unwrap();
                let high = *nums[li as usize..=ri as usize].iter().max().unwrap();
                let len = ri - li + 1;
                let d = (high - low) as usize / (len - 1) as usize;

                if high == low {
                    true
                } else if (high - low) % (len - 1) != 0 {
                    false
                } else {
                    let mut n = vec![false; len as usize];
                    for j in li as usize..=ri as usize {
                        if ((nums[j] - low) as usize % d != 0)
                            || (n[(nums[j] - low) as usize / d] == true)
                        {
                            return false;
                        }
                        n[(nums[j] - low) as usize / d] = true;
                    }
                    true
                }
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1630_example_1() {
        let nums = vec![4, 6, 5, 9, 3, 7];
        let l = vec![0, 0, 2];
        let r = vec![2, 3, 5];

        let result = vec![true, false, true];

        assert_eq!(Solution::check_arithmetic_subarrays(nums, l, r), result);
    }

    #[test]
    fn test_1630_example_2() {
        let nums = vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10];
        let l = vec![0, 1, 6, 4, 8, 7];
        let r = vec![4, 4, 9, 7, 9, 10];

        let result = vec![false, true, false, false, true, true];

        assert_eq!(Solution::check_arithmetic_subarrays(nums, l, r), result);
    }
}

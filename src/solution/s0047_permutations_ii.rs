/**
 * [47] Permutations II
 *
 * Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,2]
 * Output:
 * [[1,1,2],
 *  [1,2,1],
 *  [2,1,1]]
 *
 * Example 2:
 *
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 8
 * 	-10 <= nums[i] <= 10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations-ii/
// discuss: https://leetcode.com/problems/permutations-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/permutations-ii/discuss/714100/Rust-Backtracking
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.is_empty() {
            true => vec![Vec::new()],
            false => {
                let mut nums = nums;
                nums.sort_unstable();
                let mut ret = Vec::new();

                for i in 0..nums.len() {
                    if i == 0 || nums[i] != nums[i - 1] {
                        let mut nums_clone = nums.clone();
                        nums_clone.remove(i);

                        let mut back_ret = Self::permute_unique(nums_clone);

                        for arr in &mut back_ret {
                            arr.push(nums[i]);
                        }
                        ret.append(&mut back_ret);
                    }
                }

                ret
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0047_example_1() {
        let nums = vec![1, 1, 2];

        assert_eq_sorted!(
            Solution::permute_unique(nums),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        )
    }

    #[test]
    fn test_0047_example_2() {
        let nums = vec![1, 2, 3];

        assert_eq_sorted!(
            Solution::permute_unique(nums),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        )
    }
}

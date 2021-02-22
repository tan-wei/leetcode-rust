/**
 * [46] Permutations
 *
 * Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
 *  
 * Example 1:
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * Example 2:
 * Input: nums = [0,1]
 * Output: [[0,1],[1,0]]
 * Example 3:
 * Input: nums = [1]
 * Output: [[1]]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 6
 * 	-10 <= nums[i] <= 10
 * 	All the integers of nums are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations/
// discuss: https://leetcode.com/problems/permutations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/permutations/discuss/505394/Rust-0ms.-Iterators-madness
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            vec![nums]
        } else {
            (0..nums.len())
                .flat_map(|index| {
                    let (left, value, right) = (&nums[..index], nums[index], &nums[index + 1..]);
                    Self::permute(left.iter().chain(right.iter()).cloned().collect())
                        .into_iter()
                        .map(move |tail| std::iter::once(value).chain(tail.into_iter()).collect())
                })
                .collect()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0046_example_1() {
        let nums = vec![1, 2, 3];

        assert_eq_sorted!(
            Solution::permute(nums),
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

    #[test]
    fn test_0046_example_2() {
        let nums = vec![0, 1];

        assert_eq_sorted!(Solution::permute(nums), vec![vec![0, 1], vec![1, 0]])
    }

    #[test]
    fn test_0046_example_3() {
        let nums = vec![1];

        assert_eq_sorted!(Solution::permute(nums), vec![vec![1]]);
    }
}

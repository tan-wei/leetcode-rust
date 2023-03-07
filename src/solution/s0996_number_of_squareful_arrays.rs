/**
 * [0996] Number of Squareful Arrays
 *
 * An array is squareful if the sum of every pair of adjacent elements is a perfect square.
 * Given an integer array nums, return the number of permutations of nums that are squareful.
 * Two permutations perm1 and perm2 are different if there is some index i such that perm1[i] != perm2[i].
 *  
 * Example 1:
 *
 * Input: nums = [1,17,8]
 * Output: 2
 * Explanation: [1,8,17] and [17,8,1] are the valid permutations.
 *
 * Example 2:
 *
 * Input: nums = [2,2,2]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 12
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-squareful-arrays/
// discuss: https://leetcode.com/problems/number-of-squareful-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // credit: https://leetcode.com/problems/number-of-squareful-arrays/solutions/537277/rust-bfs-0ms-2-1m/
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut map = vec![vec![]; nums.len()];
        let mut visited = std::collections::HashSet::new();
        fn is_perfect_square(x: i32) -> bool {
            ((x as f64).sqrt() as i32).pow(2) == x
        }

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }

                if is_perfect_square(nums[i] + nums[j]) {
                    map[i].push(j);
                }
            }
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..nums.len() {
            queue.push_back(vec![i]);
        }

        let mut result = 0;

        while let Some(sub) = queue.pop_front() {
            if sub.len() == nums.len() {
                result += 1;
                continue;
            }

            let options = &map[sub[sub.len() - 1]];
            for v in options {
                if sub[0..].contains(&v) {
                    continue;
                }

                let mut next = sub[0..].to_vec();
                next.push(*v);

                let footprint =
                    format!("{:?}", next.iter().map(|x| nums[*x]).collect::<Vec<i32>>());

                if !visited.contains(&footprint) {
                    visited.insert(footprint);
                    queue.push_back(next);
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0996_example_1() {
        let nums = vec![1, 17, 8];
        let result = 2;

        assert_eq!(Solution::num_squareful_perms(nums), result);
    }

    #[test]
    fn test_0996_example_2() {
        let nums = vec![2, 2, 2];
        let result = 1;

        assert_eq!(Solution::num_squareful_perms(nums), result);
    }
}

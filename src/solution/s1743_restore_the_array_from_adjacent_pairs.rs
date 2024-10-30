/**
 * [1743] Restore the Array From Adjacent Pairs
 *
 * There is an integer array nums that consists of n unique elements, but you have forgotten it. However, you do remember every pair of adjacent elements in nums.
 * You are given a 2D integer array adjacentPairs of size n - 1 where each adjacentPairs[i] = [ui, vi] indicates that the elements ui and vi are adjacent in nums.
 * It is guaranteed that every adjacent pair of elements nums[i] and nums[i+1] will exist in adjacentPairs, either as [nums[i], nums[i+1]] or [nums[i+1], nums[i]]. The pairs can appear in any order.
 * Return the original array nums. If there are multiple solutions, return any of them.
 *  
 * Example 1:
 *
 * Input: adjacentPairs = [[2,1],[3,4],[3,2]]
 * Output: [1,2,3,4]
 * Explanation: This array has all its adjacent pairs in adjacentPairs.
 * Notice that adjacentPairs[i] may not be in left-to-right order.
 *
 * Example 2:
 *
 * Input: adjacentPairs = [[4,-2],[1,4],[-3,1]]
 * Output: [-2,4,1,-3]
 * Explanation: There can be negative numbers.
 * Another solution is [-3,1,4,-2], which would also be accepted.
 *
 * Example 3:
 *
 * Input: adjacentPairs = [[100000,-100000]]
 * Output: [100000,-100000]
 *
 *  
 * Constraints:
 *
 * 	nums.length == n
 * 	adjacentPairs.length == n - 1
 * 	adjacentPairs[i].length == 2
 * 	2 <= n <= 10^5
 * 	-10^5 <= nums[i], ui, vi <= 10^5
 * 	There exists some nums that has adjacentPairs as its pairs.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/
// discuss: https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mp = std::collections::HashMap::<i32, Vec<i32>>::new();

        for p in adjacent_pairs {
            mp.entry(p[0]).or_insert(vec![]).push(p[1]);
            mp.entry(p[1]).or_insert(vec![]).push(p[0]);
        }

        let mut start = 0;
        for (k, v) in &mp {
            if v.len() == 2 {
                continue;
            }
            start = *k;
            break;
        }

        let mut s = std::collections::HashSet::new();
        let mut result = vec![start];
        s.insert(start);

        while mp.contains_key(&start) {
            let mut v = mp.get(&start).unwrap();
            let mut done = true;
            for u in mp.get(&start).unwrap() {
                if s.contains(&u) {
                    continue;
                }
                start = *u;
                result.push(*u);
                s.insert(*u);
                done = false;
                break;
            }
            if done {
                break;
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
    fn test_1743_example_1() {
        let adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];

        let result = vec![1, 2, 3, 4];

        assert_eq_sorted!(Solution::restore_array(adjacent_pairs), result);
    }

    #[test]
    fn test_1743_example_2() {
        let adjacent_pairs = vec![vec![4, -2], vec![1, 4], vec![-3, 1]];

        let result = vec![-2, 4, 1, -3];

        assert_eq_sorted!(Solution::restore_array(adjacent_pairs), result);
    }

    #[test]
    fn test_1743_example_3() {
        let adjacent_pairs = vec![vec![100000, -100000]];

        let result = vec![100000, -100000];

        assert_eq_sorted!(Solution::restore_array(adjacent_pairs), result);
    }
}

/**
 * [1345] Jump Game IV
 *
 * Given an array of integers arr, you are initially positioned at the first index of the array.
 * In one step you can jump from index i to index:
 *
 * 	i + 1 where: i + 1 < arr.length.
 * 	i - 1 where: i - 1 >= 0.
 * 	j where: arr[i] == arr[j] and i != j.
 *
 * Return the minimum number of steps to reach the last index of the array.
 * Notice that you can not jump outside of the array at any time.
 *  
 * Example 1:
 *
 * Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
 * Output: 3
 * Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
 *
 * Example 2:
 *
 * Input: arr = [7]
 * Output: 0
 * Explanation: Start index is the last index. You do not need to jump.
 *
 * Example 3:
 *
 * Input: arr = [7,6,9,6,9,6,9,7]
 * Output: 1
 * Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 5 * 10^4
 * 	-10^8 <= arr[i] <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-iv/
// discuss: https://leetcode.com/problems/jump-game-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        for (i, &x) in arr.iter().enumerate() {
            hash.entry(x).or_insert(Vec::new()).push(i);
        }
        let mut visited = vec![false; arr.len()];
        visited[0] = true;

        let mut curr = vec![0];
        let mut next = Vec::new();
        for step in 0.. {
            for i in curr.drain(..) {
                if i == arr.len() - 1 {
                    return step;
                }
                for j in hash
                    .remove(&arr[i])
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .chain(vec![i.saturating_sub(1), i + 1].into_iter())
                {
                    if !visited[j] {
                        next.push(j);
                        visited[j] = true;
                    }
                }
            }
            std::mem::swap(&mut curr, &mut next);
        }
        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1345_example_1() {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];

        let result = 3;

        assert_eq!(Solution::min_jumps(arr), result);
    }

    #[test]
    fn test_1345_example_2() {
        let arr = vec![7];

        let result = 0;

        assert_eq!(Solution::min_jumps(arr), result);
    }

    #[test]
    fn test_1345_example_3() {
        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];

        let result = 1;

        assert_eq!(Solution::min_jumps(arr), result);
    }
}

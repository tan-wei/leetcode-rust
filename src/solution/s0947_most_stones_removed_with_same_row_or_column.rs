/**
 * [0947] Most Stones Removed with Same Row or Column
 *
 * On a 2D plane, we place n stones at some integer coordinate points. Each coordinate point may have at most one stone.
 * A stone can be removed if it shares either the same row or the same column as another stone that has not been removed.
 * Given an array stones of length n where stones[i] = [xi, yi] represents the location of the i^th stone, return the largest possible number of stones that can be removed.
 *  
 * Example 1:
 *
 * Input: stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
 * Output: 5
 * Explanation: One way to remove 5 stones is as follows:
 * 1. Remove stone [2,2] because it shares the same row as [2,1].
 * 2. Remove stone [2,1] because it shares the same column as [0,1].
 * 3. Remove stone [1,2] because it shares the same row as [1,0].
 * 4. Remove stone [1,0] because it shares the same column as [0,0].
 * 5. Remove stone [0,1] because it shares the same row as [0,0].
 * Stone [0,0] cannot be removed since it does not share a row/column with another stone still on the plane.
 *
 * Example 2:
 *
 * Input: stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
 * Output: 3
 * Explanation: One way to make 3 moves is as follows:
 * 1. Remove stone [2,2] because it shares the same row as [2,0].
 * 2. Remove stone [2,0] because it shares the same column as [0,0].
 * 3. Remove stone [0,2] because it shares the same row as [0,0].
 * Stones [0,0] and [1,1] cannot be removed since they do not share a row/column with another stone still on the plane.
 *
 * Example 3:
 *
 * Input: stones = [[0,0]]
 * Output: 0
 * Explanation: [0,0] is the only stone on the plane, so you cannot remove it.
 *
 *  
 * Constraints:
 *
 * 	1 <= stones.length <= 1000
 * 	0 <= xi, yi <= 10^4
 * 	No two stones are at the same coordinate point.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
// discuss: https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut stones = stones;
        let mut result = 0;
        while !stones.is_empty() {
            let mut q = std::collections::VecDeque::new();
            q.push_front(stones.pop().unwrap());

            while let Some(s) = q.pop_back() {
                let mut i = 0;
                while i < stones.len() {
                    if s[0] == stones[i][0] || s[1] == stones[i][1] {
                        q.push_front(stones.remove(i));
                        result += 1;
                    } else {
                        i += 1;
                    }
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
    fn test_0947_example_1() {
        let stones = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        let result = 5;

        assert_eq!(Solution::remove_stones(stones), result);
    }

    #[test]
    fn test_0947_example_2() {
        let stones = vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        let result = 3;

        assert_eq!(Solution::remove_stones(stones), result);
    }

    #[test]
    fn test_0947_example_3() {
        let stones = vec![vec![0, 0]];
        let result = 0;

        assert_eq!(Solution::remove_stones(stones), result);
    }
}

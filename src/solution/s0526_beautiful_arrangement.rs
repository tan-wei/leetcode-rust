/**
 * [0526] Beautiful Arrangement
 *
 * Suppose you have n integers labeled 1 through n. A permutation of those n integers perm (1-indexed) is considered a beautiful arrangement if for every i (1 <= i <= n), either of the following is true:
 *
 * 	perm[i] is divisible by i.
 * 	i is divisible by perm[i].
 *
 * Given an integer n, return the number of the beautiful arrangements that you can construct.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: 2
 * Explanation:
 * The first beautiful arrangement is [1,2]:
 *     - perm[1] = 1 is divisible by i = 1
 *     - perm[2] = 2 is divisible by i = 2
 * The second beautiful arrangement is [2,1]:
 *     - perm[1] = 2 is divisible by i = 1
 *     - i = 2 is divisible by perm[2] = 1
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/beautiful-arrangement/
// discuss: https://leetcode.com/problems/beautiful-arrangement/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut v = (1..=n as usize).collect::<Vec<_>>();
        Self::backtrack(&mut v, n as usize)
    }

    fn backtrack(v: &mut Vec<usize>, i: usize) -> i32 {
        match i {
            i if i >= 2 => {
                let mut ret = 0;
                let candidates = (0..i)
                    .filter(|&j| v[j] % i == 0 || i % v[j] == 0)
                    .collect::<Vec<_>>();
                for &j in candidates.iter() {
                    v.swap(j, i - 1);
                    ret += Solution::backtrack(v, i - 1);
                    v.swap(j, i - 1);
                }
                ret
            }
            _ => 1,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0526_example_1() {
        let n = 2;
        let result = 2;

        assert_eq!(Solution::count_arrangement(n), result);
    }

    #[test]
    fn test_0526_example_2() {
        let n = 1;
        let result = 1;

        assert_eq!(Solution::count_arrangement(n), result);
    }
}

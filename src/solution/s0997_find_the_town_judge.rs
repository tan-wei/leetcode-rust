/**
 * [0997] Find the Town Judge
 *
 * In a town, there are n people labeled from 1 to n. There is a rumor that one of these people is secretly the town judge.
 * If the town judge exists, then:
 * <ol>
 * 	The town judge trusts nobody.
 * 	Everybody (except for the town judge) trusts the town judge.
 * 	There is exactly one person that satisfies properties 1 and 2.
 * </ol>
 * You are given an array trust where trust[i] = [ai, bi] representing that the person labeled ai trusts the person labeled bi. If a trust relationship does not exist in trust array, then such a trust relationship does not exist.
 * Return the label of the town judge if the town judge exists and can be identified, or return -1 otherwise.
 *  
 * Example 1:
 *
 * Input: n = 2, trust = [[1,2]]
 * Output: 2
 *
 * Example 2:
 *
 * Input: n = 3, trust = [[1,3],[2,3]]
 * Output: 3
 *
 * Example 3:
 *
 * Input: n = 3, trust = [[1,3],[2,3],[3,1]]
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 * 	0 <= trust.length <= 10^4
 * 	trust[i].length == 2
 * 	All the pairs of trust are unique.
 * 	ai != bi
 * 	1 <= ai, bi <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-town-judge/
// discuss: https://leetcode.com/problems/find-the-town-judge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        trust
            .iter()
            .fold(vec![(0, 0); n as usize], |mut acc, x| {
                acc[(x[0] - 1) as usize].0 += 1;
                acc[(x[1] - 1) as usize].1 += 1;
                acc
            })
            .iter()
            .position(|(from, to)| from == &0 && to == &(n - 1))
            .map_or(-1, |x| (x + 1) as _)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0997_example_1() {
        let n = 2;
        let trust = vec![vec![1, 2]];
        let result = 2;

        assert_eq!(Solution::find_judge(n, trust), result);
    }

    #[test]
    fn test_0997_example_2() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3]];
        let result = 3;

        assert_eq!(Solution::find_judge(n, trust), result);
    }

    #[test]
    fn test_0997_example_3() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        let result = -1;

        assert_eq!(Solution::find_judge(n, trust), result);
    }
}

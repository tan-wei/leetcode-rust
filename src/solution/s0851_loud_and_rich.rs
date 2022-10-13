/**
 * [0851] Loud and Rich
 *
 * There is a group of n people labeled from 0 to n - 1 where each person has a different amount of money and a different level of quietness.
 * You are given an array richer where richer[i] = [ai, bi] indicates that ai has more money than bi and an integer array quiet where quiet[i] is the quietness of the i^th person. All the given data in richer are logically correct (i.e., the data will not lead you to a situation where x is richer than y and y is richer than x at the same time).
 * Return an integer array answer where answer[x] = y if y is the least quiet person (that is, the person y with the smallest value of quiet[y]) among all people who definitely have equal to or more money than the person x.
 *  
 * <strong class="example">Example 1:
 *
 * Input: richer = [[1,0],[2,1],[3,1],[3,7],[4,3],[5,3],[6,3]], quiet = [3,2,5,4,6,1,7,0]
 * Output: [5,5,2,5,4,5,6,7]
 * Explanation:
 * answer[0] = 5.
 * Person 5 has more money than 3, which has more money than 1, which has more money than 0.
 * The only person who is quieter (has lower quiet[x]) is person 7, but it is not clear if they have more money than person 0.
 * answer[7] = 7.
 * Among all people that definitely have equal to or more money than person 7 (which could be persons 3, 4, 5, 6, or 7), the person who is the quietest (has lower quiet[x]) is person 7.
 * The other answers can be filled out with similar reasoning.
 *
 * <strong class="example">Example 2:
 *
 * Input: richer = [], quiet = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	n == quiet.length
 * 	1 <= n <= 500
 * 	0 <= quiet[i] < n
 * 	All the values of quiet are unique.
 * 	0 <= richer.length <= n * (n - 1) / 2
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	All the pairs of richer are unique.
 * 	The observations in richer are all logically consistent.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/loud-and-rich/
// discuss: https://leetcode.com/problems/loud-and-rich/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/loud-and-rich/solutions/892612/rust-translated-16ms-100/
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut result = vec![-1; n];
        let mut richer2 = std::collections::HashMap::<i32, Vec<i32>>::new();
        for i in 0..n {
            richer2.insert(i as i32, Vec::<i32>::new());
        }
        for v in richer.iter() {
            richer2.entry(v[1]).or_default().push(v[0]);
        }
        for i in 0..n {
            Self::dfs_helper(&richer2, &quiet, i as i32, &mut result);
        }

        result
    }

    fn dfs_helper(
        richer2: &std::collections::HashMap<i32, Vec<i32>>,
        quiet: &[i32],
        i: i32,
        result: &mut Vec<i32>,
    ) -> i32 {
        if result[i as usize] >= 0 {
            return result[i as usize];
        }

        result[i as usize] = i;

        for &j in richer2.get(&i).unwrap() {
            if quiet[result[i as usize] as usize]
                > quiet[Self::dfs_helper(richer2, quiet, j, result) as usize]
            {
                result[i as usize] = result[j as usize];
            }
        }

        result[i as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0851_example_1() {
        let richer = vec![
            vec![1, 0],
            vec![2, 1],
            vec![3, 1],
            vec![3, 7],
            vec![4, 3],
            vec![5, 3],
            vec![6, 3],
        ];
        let quiet = vec![3, 2, 5, 4, 6, 1, 7, 0];
        let result = vec![5, 5, 2, 5, 4, 5, 6, 7];

        assert_eq!(Solution::loud_and_rich(richer, quiet), result);
    }

    #[test]
    fn test_0851_example_2() {
        let richer = vec![];
        let quiet = vec![0];
        let result = vec![0];

        assert_eq!(Solution::loud_and_rich(richer, quiet), result);
    }
}

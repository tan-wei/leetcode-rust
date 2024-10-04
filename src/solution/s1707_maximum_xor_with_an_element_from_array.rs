/**
 * [1707] Maximum XOR With an Element From Array
 *
 * You are given an array nums consisting of non-negative integers. You are also given a queries array, where queries[i] = [xi, mi].
 * The answer to the i^th query is the maximum bitwise XOR value of xi and any element of nums that does not exceed mi. In other words, the answer is max(nums[j] XOR xi) for all j such that nums[j] <= mi. If all elements in nums are larger than mi, then the answer is -1.
 * Return an integer array answer where answer.length == queries.length and answer[i] is the answer to the i^th query.
 *  
 * Example 1:
 *
 * Input: nums = [0,1,2,3,4], queries = [[3,1],[1,3],[5,6]]
 * Output: [3,3,7]
 * Explanation:
 * 1) 0 and 1 are the only two integers not greater than 1. 0 XOR 3 = 3 and 1 XOR 3 = 2. The larger of the two is 3.
 * 2) 1 XOR 2 = 3.
 * 3) 5 XOR 2 = 7.
 *
 * Example 2:
 *
 * Input: nums = [5,2,4,6,6,3], queries = [[12,4],[8,1],[6,3]]
 * Output: [15,-1,5]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length, queries.length <= 10^5
 * 	queries[i].length == 2
 * 	0 <= nums[j], xi, mi <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-xor-with-an-element-from-array/
// discuss: https://leetcode.com/problems/maximum-xor-with-an-element-from-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug, Clone, Default)]
struct Node {
    children: [Option<Box<Node>>; 2],
}

impl Node {
    pub fn insert(&mut self, n: i32) {
        do_insert(self, n, 31);
    }
}

fn do_insert(node: &mut Node, num: i32, bit: u32) {
    let b = ((num & (1 << bit)) != 0) as usize;
    if node.children[b].is_none() {
        node.children[b] = Some(Box::new(Node::default()));
    }

    if bit > 0 {
        do_insert(node.children[b].as_mut().unwrap(), num, bit - 1);
    }
}

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-xor-with-an-element-from-array/solutions/1762758/rust-trie-solution/
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result_indexes = std::collections::HashMap::new();
        for (idx, query) in queries.iter().enumerate() {
            result_indexes
                .entry((query[0], query[1]))
                .or_insert(vec![])
                .push(idx);
        }

        let mut nums = nums.clone();
        let mut queries = queries.clone();

        nums.sort_unstable();
        queries.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut result = vec![0; queries.len()];
        let mut trie = Node::default();

        let mut next_idx = 0;

        let mut last_query = None;

        for query in queries.iter() {
            if Some(query) == last_query {
                continue;
            }
            last_query = Some(query);

            let val = query[0];
            let lim = query[1];

            for idx in next_idx..nums.len() {
                let n = nums[idx];
                if n > lim {
                    break;
                }

                trie.insert(n);
                next_idx = idx + 1;
            }

            let mut xored = -1;
            if next_idx != 0 {
                let mut node = &trie;
                xored = 0;

                for bit in (0..32).rev() {
                    let is_set = (val & (1 << bit)) != 0;
                    match &node.children[(is_set ^ true) as usize] {
                        None => {
                            node = node.children[is_set as usize].as_ref().unwrap();
                        }

                        Some(child) => {
                            xored |= 1 << bit;
                            node = child;
                        }
                    }
                }
            }

            for &idx in result_indexes.get(&(val, lim)).unwrap() {
                result[idx] = xored;
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
    fn test_1707_example_1() {
        let nums = vec![0, 1, 2, 3, 4];
        let queries = vec![vec![3, 1], vec![1, 3], vec![5, 6]];

        let result = vec![3, 3, 7];

        assert_eq!(Solution::maximize_xor(nums, queries), result);
    }

    #[test]
    fn test_1707_example_2() {
        let nums = vec![5, 2, 4, 6, 6, 3];
        let queries = vec![vec![12, 4], vec![8, 1], vec![6, 3]];

        let result = vec![15, -1, 5];

        assert_eq!(Solution::maximize_xor(nums, queries), result);
    }
}

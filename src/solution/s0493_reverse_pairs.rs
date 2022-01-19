/**
 * [0493] Reverse Pairs
 *
 * Given an integer array nums, return the number of reverse pairs in the array.
 * A reverse pair is a pair (i, j) where 0 <= i < j < nums.length and nums[i] > 2 * nums[j].
 *  
 * Example 1:
 * Input: nums = [1,3,2,3,1]
 * Output: 2
 * Example 2:
 * Input: nums = [2,4,3,5,1]
 * Output: 3
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-pairs/
// discuss: https://leetcode.com/problems/reverse-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

pub struct BinaryIndexTree {
    n: usize,
    nums: Vec<i32>,
}

impl BinaryIndexTree {
    pub fn new(length: usize) -> Self {
        Self {
            n: length + 1,
            nums: vec![0; length + 1],
        }
    }

    pub fn update(&mut self, mut index: usize, delta: i32) {
        while index < self.n {
            self.nums[index] += delta;
            index = (index | (index - 1)) + 1;
        }
    }

    pub fn query(&self, mut index: usize) -> i32 {
        let mut sum: i32 = 0;
        while index > 0 {
            sum += self.nums[index];
            index &= index - 1;
        }
        sum
    }
}

impl Solution {
    // Inspired: https://leetcode.com/problems/reverse-pairs/discuss/97268/General-principles-behind-problems-similar-to-%22Reverse-Pairs%22
    // Credit: https://leetcode.com/problems/reverse-pairs/discuss/1400758/RustPython-Divide-and-Conquer-with-Binary-Index-Tree
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let rank: std::collections::HashMap<i64, usize> = {
            let mut sorted = Vec::new();
            for &x in &nums {
                sorted.push(x as i64);
                sorted.push(2 * x as i64);
            }
            sorted.sort();
            sorted.dedup();
            sorted
        }
        .into_iter()
        .enumerate()
        .map(|(i, num)| (num, i + 1))
        .collect();

        let mut tree = BinaryIndexTree::new(rank.keys().len());
        let mut result = 0;

        for &n in nums.iter().rev() {
            let num = n as i64;
            result += tree.query(rank[&num] - 1);
            tree.update(rank[&(num * 2)], 1);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0493_example_1() {
        let nums = vec![1, 3, 2, 3, 1];
        let result = 2;

        assert_eq!(Solution::reverse_pairs(nums), result);
    }

    fn test_0493_example_2() {
        let nums = vec![2, 4, 3, 5, 1];
        let result = 3;

        assert_eq!(Solution::reverse_pairs(nums), result);
    }
}

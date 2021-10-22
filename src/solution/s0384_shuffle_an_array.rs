/**
 * [384] Shuffle an Array
 *
 * Given an integer array nums, design an algorithm to randomly shuffle the array. All permutations of the array should be equally likely as a result of the shuffling.
 * Implement the Solution class:
 *
 * 	Solution(int[] nums) Initializes the object with the integer array nums.
 * 	int[] reset() Resets the array to its original configuration and returns it.
 * 	int[] shuffle() Returns a random shuffling of the array.
 *
 *  
 * Example 1:
 *
 * Input
 * ["Solution", "shuffle", "reset", "shuffle"]
 * [[[1, 2, 3]], [], [], []]
 * Output
 * [null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]
 * Explanation
 * Solution solution = new Solution([1, 2, 3]);
 * solution.shuffle();    // Shuffle the array [1,2,3] and return its result.
 *                        // Any permutation of [1,2,3] must be equally likely to be returned.
 *                        // Example: return [3, 1, 2]
 * solution.reset();      // Resets the array back to its original configuration [1,2,3]. Return [1, 2, 3]
 * solution.shuffle();    // Returns the random shuffling of array [1,2,3]. Example: return [1, 3, 2]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 200
 * 	-10^6 <= nums[i] <= 10^6
 * 	All the elements of nums are unique.
 * 	At most 5 * 10^4 calls in total will be made to reset and shuffle.
 *
 */
// problem: https://leetcode.com/problems/shuffle-an-array/
// discuss: https://leetcode.com/problems/shuffle-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use rand::distributions::Uniform;
use rand::prelude::*;

struct Solution {
    origin: Vec<i32>,
    shuffled: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            origin: nums.clone(),
            shuffled: nums,
            rng: thread_rng(),
        }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.shuffled = self.origin.clone();
        self.origin.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let n = self.shuffled.len();
        // NOTE: Fisher–Yates shuffle
        for i in 0..(n - 1) {
            let uniform = Uniform::from(i..n);
            let j = self.rng.sample(uniform);
            self.shuffled.swap(i, j);
        }
        self.shuffled.clone()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0384_example_1() {
        let nums = vec![1, 2, 3];
        let mut obj = Solution::new(nums);
        let ret_1: Vec<i32> = obj.reset();
        let ret_2: Vec<i32> = obj.shuffle();
    }
}

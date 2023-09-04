/**
 * [1298] Maximum Candies You Can Get from Boxes
 *
 * You have n boxes labeled from 0 to n - 1. You are given four arrays: status, candies, keys, and containedBoxes where:
 *
 * 	status[i] is 1 if the i^th box is open and 0 if the i^th box is closed,
 * 	candies[i] is the number of candies in the i^th box,
 * 	keys[i] is a list of the labels of the boxes you can open after opening the i^th box.
 * 	containedBoxes[i] is a list of the boxes you found inside the i^th box.
 *
 * You are given an integer array initialBoxes that contains the labels of the boxes you initially have. You can take all the candies in any open box and you can use the keys in it to open new boxes and you also can use the boxes you find in it.
 * Return the maximum number of candies you can get following the rules above.
 *
 * Example 1:
 *
 * Input: status = [1,0,1,0], candies = [7,5,4,100], keys = [[],[],[1],[]], containedBoxes = [[1,2],[3],[],[]], initialBoxes = [0]
 * Output: 16
 * Explanation: You will be initially given box 0. You will find 7 candies in it and boxes 1 and 2.
 * Box 1 is closed and you do not have a key for it so you will open box 2. You will find 4 candies and a key to box 1 in box 2.
 * In box 1, you will find 5 candies and box 3 but you will not find a key to box 3 so box 3 will remain closed.
 * Total number of candies collected = 7 + 4 + 5 = 16 candy.
 *
 * Example 2:
 *
 * Input: status = [1,0,0,0,0,0], candies = [1,1,1,1,1,1], keys = [[1,2,3,4,5],[],[],[],[],[]], containedBoxes = [[1,2,3,4,5],[],[],[],[],[]], initialBoxes = [0]
 * Output: 6
 * Explanation: You have initially box 0. Opening it you can find boxes 1,2,3,4 and 5 and their keys.
 * The total number of candies will be 6.
 *
 *
 * Constraints:
 *
 * 	n == status.length == candies.length == keys.length == containedBoxes.length
 * 	1 <= n <= 1000
 * 	status[i] is either 0 or 1.
 * 	1 <= candies[i] <= 1000
 * 	0 <= keys[i].length <= n
 * 	0 <= keys[i][j] < n
 * 	All values of keys[i] are unique.
 * 	0 <= containedBoxes[i].length <= n
 * 	0 <= containedBoxes[i][j] < n
 * 	All values of containedBoxes[i] are unique.
 * 	Each box is contained in one box at most.
 * 	0 <= initialBoxes.length <= n
 * 	0 <= initialBoxes[i] < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/
// discuss: https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/solutions/876638/rust-translated-16ms-100/
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut status = status;
        let mut result = 0;
        let mut q = std::collections::VecDeque::<i32>::new();
        for i in initial_boxes {
            status[i as usize] += 5000;
            if status[i as usize] > 5000 {
                q.push_back(i as i32);
            }
        }
        while let Some(b) = q.pop_front() {
            result += candies[b as usize];
            for &i in &keys[b as usize] {
                status[i as usize] += 5;
                if status[i as usize] == 5005 {
                    q.push_back(i as i32);
                }
            }
            for &i in &contained_boxes[b as usize] {
                status[i as usize] += 5000;
                if status[i as usize] > 5000 {
                    q.push_back(i as i32);
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
    fn test_1298_example_1() {
        let status = vec![1, 0, 1, 0];
        let candies = vec![7, 5, 4, 100];
        let keys = vec![vec![], vec![], vec![1], vec![]];
        let contained_boxes = vec![vec![1, 2], vec![3], vec![], vec![]];
        let initial_boxes = vec![0];
        let result = 16;

        assert_eq!(
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
            result
        );
    }

    #[test]
    fn test_1298_example_2() {
        let status = vec![1, 0, 0, 0, 0, 0];
        let candies = vec![1, 1, 1, 1, 1, 1];
        let keys = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
        let contained_boxes = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
        let initial_boxes = vec![0];
        let result = 6;

        assert_eq!(
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
            result
        );
    }
}

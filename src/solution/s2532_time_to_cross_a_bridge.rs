/**
 * [2532] Time to Cross a Bridge
 *
 * There are k workers who want to move n boxes from the right (old) warehouse to the left (new) warehouse. You are given the two integers n and k, and a 2D integer array time of size k x 4 where time[i] = [righti, picki, lefti, puti].
 * The warehouses are separated by a river and connected by a bridge. Initially, all k workers are waiting on the left side of the bridge. To move the boxes, the i^th worker can do the following:
 *
 * 	Cross the bridge to the right side in righti minutes.
 * 	Pick a box from the right warehouse in picki minutes.
 * 	Cross the bridge to the left side in lefti minutes.
 * 	Put the box into the left warehouse in puti minutes.
 *
 * The i^th worker is less efficient than the j^th worker if either condition is met:
 *
 * 	lefti + righti > leftj + rightj
 * 	lefti + righti == leftj + rightj and i > j
 *
 * The following rules regulate the movement of the workers through the bridge:
 *
 * 	Only one worker can use the bridge at a time.
 * 	When the bridge is unused prioritize the least efficient worker (who have picked up the box) on the right side to cross. If not, prioritize the least efficient worker on the left side to cross.
 * 	If enough workers have already been dispatched from the left side to pick up all the remaining boxes, no more workers will be sent from the left side.
 *
 * Return the elapsed minutes at which the last box reaches the left side of the bridge.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">n = 1, k = 3, time = [[1,1,2,1],[1,1,3,1],[1,1,4,1]]</span>
 * Output: <span class="example-io">6</span>
 * Explanation:
 *
 * From 0 to 1 minutes: worker 2 crosses the bridge to the right.
 * From 1 to 2 minutes: worker 2 picks up a box from the right warehouse.
 * From 2 to 6 minutes: worker 2 crosses the bridge to the left.
 * From 6 to 7 minutes: worker 2 puts a box at the left warehouse.
 * The whole process ends after 7 minutes. We return 6 because the problem asks for the instance of time at which the last worker reaches the left side of the bridge.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">n = 3, k = 2, time =</span> [[1,5,1,8],[10,10,10,10]]
 * Output: 37
 * Explanation:
 *
 * <img src="https://assets.leetcode.com/uploads/2024/11/21/378539249-c6ce3c73-40e7-4670-a8b5-7ddb9abede11.png" style="width: 450px; height: 176px;" />
 *
 * The last box reaches the left side at 37 seconds. Notice, how we do not put the last boxes down, as that would take more time, and they are already on the left with the workers.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= n, k <= 10^4
 * 	time.length == k
 * 	time[i].length == 4
 * 	1 <= lefti, picki, righti, puti <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/time-to-cross-a-bridge/
// discuss: https://leetcode.com/problems/time-to-cross-a-bridge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2532_example_1() {
        let n = 1;
        let k = 3;
        let time = vec![vec![1, 1, 2, 1], vec![1, 1, 3, 1], vec![1, 1, 4, 1]];

        let result = 6;

        assert_eq!(Solution::find_crossing_time(n, k, time), result);
    }

    #[test]
    #[ignore]
    fn test_2532_example_2() {
        let n = 3;
        let k = 2;
        let time = vec![vec![1, 5, 1, 8], vec![10, 10, 10, 10]];

        let result = 37;

        assert_eq!(Solution::find_crossing_time(n, k, time), result);
    }
}

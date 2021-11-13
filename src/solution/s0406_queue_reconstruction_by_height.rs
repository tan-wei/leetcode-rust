/**
 * [0406] Queue Reconstruction by Height
 *
 * You are given an array of people, people, which are the attributes of some people in a queue (not necessarily in order). Each people[i] = [hi, ki] represents the i^th person of height hi with exactly ki other people in front who have a height greater than or equal to hi.
 * Reconstruct and return the queue that is represented by the input array people. The returned queue should be formatted as an array queue, where queue[j] = [hj, kj] is the attributes of the j^th person in the queue (queue[0] is the person at the front of the queue).
 *  
 * Example 1:
 *
 * Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
 * Output: [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
 * Explanation:
 * Person 0 has height 5 with no other people taller or the same height in front.
 * Person 1 has height 7 with no other people taller or the same height in front.
 * Person 2 has height 5 with two persons taller or the same height in front, which is person 0 and 1.
 * Person 3 has height 6 with one person taller or the same height in front, which is person 1.
 * Person 4 has height 4 with four people taller or the same height in front, which are people 0, 1, 2, and 3.
 * Person 5 has height 7 with one person taller or the same height in front, which is person 1.
 * Hence [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] is the reconstructed queue.
 *
 * Example 2:
 *
 * Input: people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
 * Output: [[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]
 *
 *  
 * Constraints:
 *
 * 	1 <= people.length <= 2000
 * 	0 <= hi <= 10^6
 * 	0 <= ki < people.length
 * 	It is guaranteed that the queue can be reconstructed.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/queue-reconstruction-by-height/
// discuss: https://leetcode.com/problems/queue-reconstruction-by-height/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by_key(|p| (-p[0], p[1]));
        let mut queue = Vec::with_capacity(people.len());
        for person in people.iter() {
            queue.insert(person[1] as usize, person.to_vec());
        }
        queue
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0406_example_1() {
        let people = vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ];
        let result = vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1],
        ];

        assert_eq!(Solution::reconstruct_queue(people), result);
    }

    #[test]
    fn test_0406_example_2() {
        let people = vec![
            vec![6, 0],
            vec![5, 0],
            vec![4, 0],
            vec![3, 2],
            vec![2, 2],
            vec![1, 4],
        ];
        let result = vec![
            vec![4, 0],
            vec![5, 0],
            vec![2, 2],
            vec![3, 2],
            vec![1, 4],
            vec![6, 0],
        ];

        assert_eq!(Solution::reconstruct_queue(people), result);
    }
}

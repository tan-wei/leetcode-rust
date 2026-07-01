/**
 * [2526] Find Consecutive Integers from a Data Stream
 *
 * For a stream of integers, implement a data structure that checks if the last k integers parsed in the stream are equal to value.
 * Implement the DataStream class:
 *
 * 	DataStream(int value, int k) Initializes the object with an empty integer stream and the two integers value and k.
 * 	boolean consec(int num) Adds num to the stream of integers. Returns true if the last k integers are equal to value, and false otherwise. If there are less than k integers, the condition does not hold true, so returns false.
 *
 *  
 * Example 1:
 *
 * Input
 * ["DataStream", "consec", "consec", "consec", "consec"]
 * [[4, 3], [4], [4], [4], [3]]
 * Output
 * [null, false, false, true, false]
 * Explanation
 * DataStream dataStream = new DataStream(4, 3); //value = 4, k = 3
 * dataStream.consec(4); // Only 1 integer is parsed, so returns False.
 * dataStream.consec(4); // Only 2 integers are parsed.
 *                       // Since 2 is less than k, returns False.
 * dataStream.consec(4); // The 3 integers parsed are all equal to value, so returns True.
 * dataStream.consec(3); // The last k integers parsed in the stream are [4,4,3].
 *                       // Since 3 is not equal to value, it returns False.
 *
 *  
 * Constraints:
 *
 * 	1 <= value, num <= 10^9
 * 	1 <= k <= 10^5
 * 	At most 10^5 calls will be made to consec.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/
// discuss: https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/solutions/3024180/just-a-runnable-solution-by-ssrlive-1hsb/

struct DataStream {
    value: i32,
    k: usize,
    queue: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self {
            value,
            k: k as usize,
            queue: std::collections::VecDeque::new(),
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        self.queue.push_back(num);
        if self.queue.len() > self.k {
            self.queue.pop_front();
        }
        self.queue.len() == self.k && self.queue.iter().all(|&x| x == self.value)
    }
}

/**
 * Your DataStream object will be instantiated and called as such:
 * let obj = DataStream::new(value, k);
 * let ret_1: bool = obj.consec(num);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2526_example_1() {
        let mut data_stream = DataStream::new(4, 3); //value = 4, k = 3 
        assert_eq!(data_stream.consec(4), false); // Only 1 integer is parsed, so returns False. 
        assert_eq!(data_stream.consec(4), false); // Only 2 integers are parsed.
        // Since 2 is less than k, returns False.

        assert_eq!(data_stream.consec(4), true); // The 3 integers parsed are all equal to value, so returns True. 
        assert_eq!(data_stream.consec(3), false); // The last k integers parsed in the stream are [4,4,3].
        // Since 3 is not equal to value, it returns False.
    }
}

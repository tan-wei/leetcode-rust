/**
 * [352] Data Stream as Disjoint Intervals
 *
 * Given a data stream input of non-negative integers a1, a2, ..., an, summarize the numbers seen so far as a list of disjoint intervals.
 * Implement the SummaryRanges class:
 *
 * 	SummaryRanges() Initializes the object with an empty stream.
 * 	void addNum(int val) Adds the integer val to the stream.
 * 	int[][] getIntervals() Returns a summary of the integers in the stream currently as a list of disjoint intervals [starti, endi].
 *
 *  
 * Example 1:
 *
 * Input
 * ["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
 * [[], [1], [], [3], [], [7], [], [2], [], [6], []]
 * Output
 * [null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]
 * Explanation
 * SummaryRanges summaryRanges = new SummaryRanges();
 * summaryRanges.addNum(1);      // arr = [1]
 * summaryRanges.getIntervals(); // return [[1, 1]]
 * summaryRanges.addNum(3);      // arr = [1, 3]
 * summaryRanges.getIntervals(); // return [[1, 1], [3, 3]]
 * summaryRanges.addNum(7);      // arr = [1, 3, 7]
 * summaryRanges.getIntervals(); // return [[1, 1], [3, 3], [7, 7]]
 * summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
 * summaryRanges.getIntervals(); // return [[1, 3], [7, 7]]
 * summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
 * summaryRanges.getIntervals(); // return [[1, 3], [6, 7]]
 *
 *  
 * Constraints:
 *
 * 	0 <= val <= 10^4
 * 	At most 3 * 10^4 calls will be made to addNum and getIntervals.
 *
 *  
 * Follow up: What if there are lots of merges and the number of disjoint intervals is small compared to the size of the data stream?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/data-stream-as-disjoint-intervals/
// discuss: https://leetcode.com/problems/data-stream-as-disjoint-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct SummaryRanges {
    hl: std::collections::BTreeMap<i32, i32>, //store segment keyed by left point
    hr: std::collections::BTreeMap<i32, i32>, //store segment keyed by right point
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            hl: std::collections::BTreeMap::new(),
            hr: std::collections::BTreeMap::new(),
        }
    }
    fn remove(&mut self, l: i32, r: i32) {
        self.hl.remove(&l);
        self.hr.remove(&r);
    }

    fn insert(&mut self, l: i32, r: i32) {
        self.hl.insert(l, r);
        self.hr.insert(r, l);
    }

    fn add_num(&mut self, val: i32) {
        match (
            self.hr.get_key_value(&(val - 1)),
            self.hl.get_key_value(&(val + 1)),
        ) {
            (Some((&rp, &lp)), Some((&lq, &rq))) => {
                self.remove(lp, rp);
                self.remove(lq, rq);
                self.insert(lp, rq);
            }
            (Some((&rp, &lp)), None) => {
                self.remove(lp, rp);
                self.insert(lp, rp + 1);
            }
            (None, Some((&lq, &rq))) => {
                self.remove(lq, rq);
                self.insert(lq - 1, rq);
            }
            (None, None) => {
                if self
                    .hr
                    .range(val..)
                    .next()
                    .filter(|(&_, &l)| l <= val)
                    .is_none()
                {
                    self.insert(val, val);
                }
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.hl.iter().map(|(&l, &r)| vec![l, r]).collect()
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(val);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0352_example_1() {
        let mut summary_ranges = SummaryRanges::new();
        summary_ranges.add_num(1); // arr = [1]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1]]); // return [[1, 1]]
        summary_ranges.add_num(3); // arr = [1, 3]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1], vec![3, 3]]); // return [[1, 1], [3, 3]]
        summary_ranges.add_num(7); // arr = [1, 3, 7]
        assert_eq!(
            summary_ranges.get_intervals(),
            vec![vec![1, 1], vec![3, 3], vec![7, 7]]
        ); // return [[1, 1], [3, 3], [7, 7]]
        summary_ranges.add_num(2); // arr = [1, 2, 3, 7]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![7, 7]]); // return [[1, 3], [7, 7]]
        summary_ranges.add_num(6); // arr = [1, 2, 3, 6, 7]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
        // return [[1, 3], [6, 7]]
    }
}

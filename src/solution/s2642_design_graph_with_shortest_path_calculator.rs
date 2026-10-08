/**
 * [2642] Design Graph With Shortest Path Calculator
 *
 * There is a directed weighted graph that consists of n nodes numbered from 0 to n - 1. The edges of the graph are initially represented by the given array edges where edges[i] = [fromi, toi, edgeCosti] meaning that there is an edge from fromi to toi with the cost edgeCosti.
 * Implement the Graph class:
 *
 * 	Graph(int n, int[][] edges) initializes the object with n nodes and the given edges.
 * 	addEdge(int[] edge) adds an edge to the list of edges where edge = [from, to, edgeCost]. It is guaranteed that there is no edge between the two nodes before adding this one.
 * 	int shortestPath(int node1, int node2) returns the minimum cost of a path from node1 to node2. If no path exists, return -1. The cost of a path is the sum of the costs of the edges in the path.
 *
 *  
 * Example 1:
 *
 * Input
 * ["Graph", "shortestPath", "shortestPath", "addEdge", "shortestPath"]
 * [[4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]], [3, 2], [0, 3], [[1, 3, 4]], [0, 3]]
 * Output
 * [null, 6, -1, null, 6]
 * Explanation
 * Graph g = new Graph(4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
 * g.shortestPath(3, 2); // return 6. The shortest path from 3 to 2 in the first diagram above is 3 -> 0 -> 1 -> 2 with a total cost of 3 + 2 + 1 = 6.
 * g.shortestPath(0, 3); // return -1. There is no path from 0 to 3.
 * g.addEdge([1, 3, 4]); // We add an edge from node 1 to node 3, and we get the second diagram above.
 * g.shortestPath(0, 3); // return 6. The shortest path from 0 to 3 now is 0 -> 1 -> 3 with a total cost of 2 + 4 = 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 * 	0 <= edges.length <= n * (n - 1)
 * 	edges[i].length == edge.length == 3
 * 	0 <= fromi, toi, from, to, node1, node2 <= n - 1
 * 	1 <= edgeCosti, edgeCost <= 10^6
 * 	There are no repeated edges and no self-loops in the graph at any point.
 * 	At most 100 calls will be made for addEdge.
 * 	At most 100 calls will be made for shortestPath.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-graph-with-shortest-path-calculator/
// discuss: https://leetcode.com/problems/design-graph-with-shortest-path-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/design-graph-with-shortest-path-calculator/solutions/4275801/rustpython-should-be-at-most-medium-just-3aa5/

struct Graph {
    g: Vec<Vec<(usize, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut g = vec![Vec::new(); n as usize];
        for edge in edges {
            g[edge[0] as usize].push((edge[1] as usize, edge[2]))
        }
        Self { g }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.g[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let node2 = node2 as usize;
        let mut heap = std::collections::BinaryHeap::from([(0, node1 as usize)]);
        let mut all_cost = vec![i32::MAX; self.g.len()];

        while !heap.is_empty() {
            let (cost, node) = heap.pop().unwrap();
            if node == node2 {
                return -cost;
            }

            for i in 0..self.g[node].len() {
                let (node_nxt, c) = self.g[node][i];
                if -cost + c < all_cost[node_nxt] {
                    all_cost[node_nxt] = -cost + c;
                    heap.push((cost - c, node_nxt));
                }
            }
        }

        return -1;
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2642_example_1() {
        let mut g = Graph::new(
            4,
            vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]],
        );
        assert_eq!(g.shortest_path(3, 2), 6); // return 6. The shortest path from 3 to 2 in the first diagram above is 3 -> 0 -> 1 -> 2 with a total cost of 3 + 2 + 1 = 6.
        assert_eq!(g.shortest_path(0, 3), -1); // return -1. There is no path from 0 to 3.
        g.add_edge(vec![1, 3, 4]); // We add an edge from node 1 to node 3, and we get the second diagram above.
        assert_eq!(g.shortest_path(0, 3), 6); // return 6. The shortest path from 0 to 3 now is 0 -> 1 -> 3 with a total cost of 2 + 4 = 6.
    }
}

/**
 * [0913] Cat and Mouse
 *
 * A game on an undirected graph is played by two players, Mouse and Cat, who alternate turns.
 * The graph is given as follows: graph[a] is a list of all nodes b such that ab is an edge of the graph.
 * The mouse starts at node 1 and goes first, the cat starts at node 2 and goes second, and there is a hole at node 0.
 * During each player's turn, they must travel along one edge of the graph that meets where they are.  For example, if the Mouse is at node 1, it must travel to any node in graph[1].
 * Additionally, it is not allowed for the Cat to travel to the Hole (node 0.)
 * Then, the game can end in three ways:
 *
 * 	If ever the Cat occupies the same node as the Mouse, the Cat wins.
 * 	If ever the Mouse reaches the Hole, the Mouse wins.
 * 	If ever a position is repeated (i.e., the players are in the same position as a previous turn, and it is the same player's turn to move), the game is a draw.
 *
 * Given a graph, and assuming both players play optimally, return
 *
 * 	1 if the mouse wins the game,
 * 	2 if the cat wins the game, or
 * 	0 if the game is a draw.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/cat1.jpg" style="width: 300px; height: 300px;" />
 * Input: graph = [[2,5],[3],[0,4,5],[1,4,5],[2,3],[0,2,3]]
 * Output: 0
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/cat2.jpg" style="width: 200px; height: 200px;" />
 * Input: graph = [[1,3],[0],[3],[0,2]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	3 <= graph.length <= 50
 * 	1 <= graph[i].length < graph.length
 * 	0 <= graph[i][j] < graph.length
 * 	graph[i][j] != i
 * 	graph[i] is unique.
 * 	The mouse and the cat can always move.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cat-and-mouse/
// discuss: https://leetcode.com/problems/cat-and-mouse/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/cat-and-mouse/solutions/746429/clean-rust-solution/

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Player {
    Cat,
    Mouse,
}

impl Player {
    pub fn opposite(&self) -> Player {
        match self {
            Player::Cat => Player::Mouse,
            Player::Mouse => Player::Cat,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    turn: Player,
    cat_pos: i32,
    mouse_pos: i32,
}

impl State {
    const WINNING_MOUSE_POS: i32 = 0;

    pub fn new(turn: Player, cat_pos: i32, mouse_pos: i32) -> State {
        State {
            turn,
            cat_pos,
            mouse_pos,
        }
    }

    pub fn winner(&self) -> Option<Player> {
        if self.mouse_pos == State::WINNING_MOUSE_POS || self.cat_pos == State::WINNING_MOUSE_POS {
            // If either the mouse or cat falls into the hole, the mouse wins.
            Some(Player::Mouse)
        } else if self.mouse_pos == self.cat_pos {
            // If the cat catches the mouse, the cat wins.
            Some(Player::Cat)
        } else {
            // Otherwise there isn't a winner yet.
            None
        }
    }
}
impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let mut graph_set: std::collections::HashMap<State, std::collections::HashSet<State>> =
            std::collections::HashMap::new();
        let mut rgraph: std::collections::HashMap<State, std::collections::HashSet<State>> =
            std::collections::HashMap::new();
        let mut states = std::collections::HashSet::new();
        let mut state_winner: std::collections::HashMap<State, Player> =
            std::collections::HashMap::new();
        for (pos_one, pos_adjs) in graph.iter().enumerate() {
            for pos_adj in pos_adjs {
                for turn in &[Player::Mouse, Player::Cat] {
                    for cnst_pos in 0..graph.len() {
                        let (cat_pos, new_cat_pos, mouse_pos, new_mouse_pos) = match *turn {
                            Player::Cat => {
                                (pos_one as i32, *pos_adj, cnst_pos as i32, cnst_pos as i32)
                            }
                            Player::Mouse => {
                                (cnst_pos as i32, cnst_pos as i32, pos_one as i32, *pos_adj)
                            }
                        };
                        let from = State::new(*turn, cat_pos, mouse_pos);
                        let to = State::new(turn.opposite(), new_cat_pos, new_mouse_pos);
                        graph_set
                            .entry(from)
                            .or_insert(std::collections::HashSet::new())
                            .insert(to);
                        rgraph
                            .entry(to)
                            .or_insert(std::collections::HashSet::new())
                            .insert(from);
                        for state in vec![from, to] {
                            if let Some(winning_player) = state.winner() {
                                state_winner.insert(state, winning_player);
                            }
                            states.insert(state);
                        }
                    }
                }
            }
        }

        let mut degree_left = std::collections::HashMap::new();
        let mut stack = Vec::new();
        for state in states {
            if let Some(adjs) = graph_set.get(&state) {
                // The degree is the number of children left. Once all children are
                // complete, the winner of this state can be determined
                degree_left.insert(state, adjs.len() as i32);
            } else {
                degree_left.insert(state, 0);
            }

            if let Some(_) = state_winner.get(&state) {
                degree_left.insert(state, 0);
            }

            if degree_left[&state] == 0 {
                stack.push(state);
            }
        }

        while let Some(state) = stack.pop() {
            let winner = state_winner[&state];
            for &parent in rgraph
                .get(&state)
                .unwrap_or(&std::collections::HashSet::new())
            {
                if state_winner.contains_key(&parent) {
                    continue;
                }

                // If it was the winner's turn last turn, then they can make the winning move.
                if winner == parent.turn {
                    state_winner.insert(parent, winner);
                    stack.push(parent);
                    continue;
                }

                // Otherwise, decrease the `degree_left` count and if the degree is 0, this parent
                // has no winning moves, and therefore we can set the winner.
                // Safe unwrap as all states were put into `degree_left`.
                *degree_left.get_mut(&parent).unwrap() -= 1;
                if degree_left[&parent] == 0 {
                    state_winner.insert(parent, parent.turn.opposite());
                    stack.push(parent);
                }
            }
        }

        match state_winner.get(&State::new(Player::Mouse, 2, 1)) {
            None => 0,
            Some(Player::Mouse) => 1,
            Some(Player::Cat) => 2,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0913_example_1() {
        let graph = vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3],
        ];
        let result = 0;

        assert_eq!(Solution::cat_mouse_game(graph), result);
    }

    #[test]
    fn test_0913_example_2() {
        let graph = vec![vec![1, 3], vec![0], vec![3], vec![0, 2]];
        let result = 1;

        assert_eq!(Solution::cat_mouse_game(graph), result);
    }
}

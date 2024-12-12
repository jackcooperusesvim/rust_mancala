pub mod mancala;
use mancala::*;
use std::time::Instant;

fn main() {
    for depth in 2..14 {
        println!("Board with depth of {}", depth);
        let start_tree_build = Instant::now();
        let mut sboard = MancalaGameNode::default(MancalaBoard::starting_board());
        sboard.build_trees(depth);
        let duration_tree_build = start_tree_build.elapsed();
        println!("\t\tTree Building Perf: {:?}", duration_tree_build);

        let start_util_eval = Instant::now();
        sboard.evaluate_self_worth_from_children();
        let duration_util_eval = start_util_eval.elapsed();

        println!("\t\tUtility Evaluation Perf: {:?}", duration_util_eval);
        println!("\t{:?}", sboard.board);
        println!("\t{:?}", sboard.solved);
        println!("\t{:?}", sboard.utility);
        println!();
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn calculate_game(depth: usize) -> MancalaGameNode {
        let mut sboard = MancalaGameNode::default(MancalaBoard::starting_board());
        sboard.build_trees(depth);
        sboard.evaluate_self_worth_from_children();
        sboard
    }

    #[test]
    fn depth_10_test() {
        let depth = 10;
        let start_tree_build = Instant::now();
        let mut sboard = MancalaGameNode::default(MancalaBoard::starting_board());
        sboard.build_trees(depth);
        let duration_tree_build = start_tree_build.elapsed();

        let start_util_eval = Instant::now();
        sboard.evaluate_self_worth_from_children();
        let duration_util_eval = start_util_eval.elapsed();
        println!("\t\tTree Building Perf: {:?}", duration_tree_build);
        println!("\t\tUtility Evaluation Perf: {:?}", duration_util_eval);
        assert_eq!(sboard.utility, Some(0))
    }
    #[test]
    fn result_test() {
        let expected_utils: Vec<isize> = vec![1, 0, 1, 0, 1, 0, 1, 0, 0];
        (2..=10)
            .into_iter()
            .map(|depth| calculate_game(depth).utility)
            .zip(expected_utils)
            .for_each(|(util, exp_util)| assert_eq!(util.unwrap(), exp_util));
    }
}

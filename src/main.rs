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

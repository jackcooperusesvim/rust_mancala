pub mod mancala;
use mancala::*;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() {
    let depth = 11;
    println!("Board with depth of {}", depth);
    let start_tree_build = Instant::now();
    let mut sboard = MancalaGameNode::default(MancalaBoard::starting_board());
    sboard.build_trees(depth).await;
    let duration_tree_build = start_tree_build.elapsed();
    println!("\tTree Building Perf: {:?}", duration_tree_build);

    let start_util_eval = Instant::now();
    sboard.evaluate_self_worth_from_children().await;
    let duration_util_eval = start_util_eval.elapsed();

    println!("\tUtility Evaluation Perf: {:?}", duration_util_eval);
    println!("{:?}", sboard.board);
    println!("{:?}", sboard.solved);
    println!("{:?}", sboard.utility);
    println!();
    println!();
}

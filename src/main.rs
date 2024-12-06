pub mod mancala;
use mancala::*;
use std::time::{Duration, Instant};
use tokio;

#[tokio::main]
async fn main() {
    println!("Board Created");
    for i in 2..17 {
        println!("Starting Building Tree depth = {}", i);
        let start = Instant::now();
        let mut sboard = MancalaSearchBoard::default(MancalaBoard::starting_board());
        sboard.build_trees(i).await;
        let duration = start.elapsed();
        println!("Time Performance: {:?}", duration);
    }
}

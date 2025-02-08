# Rustcala

This is a simple mancala engine, which also contains a breadth-first depth-limited search algorithm to play the game. I am writing this to learn more about Rust, and this project will also be incorporated with my Machine Learning Library, also written in Rust.

## Relevant Structs
 - BoardSpace 
    This struct represents a space on the board, and can also represent a turn to be taken

 - MancalaBoard
    This struct represents a mancala board separated from game context or rules(with the exception of the player to move).

 - MancalaGameNode 
    This struct contains a MancalaBoard as well as all information needed to function as a tree-node for the MiniMax Search Algorithm.

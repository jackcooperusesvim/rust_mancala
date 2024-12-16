#Rustcala

This is a simple mancala engine, which also contains a breadth-first limited-depth search algorithm to play the game. I am writing this to learn more about Rust, and to be incorporated with my Rust-ML-Library which I am writing specifically to play this game.

## Relevant Structs
 - BoardSpace 
    This struct represents a space on the board, and can also represent a turn to be taken

 - MancalaBoard
    This struct represents a mancala board separated from game context or rules(with the exception of the player to move).

 - MancalaGameNode 
    This struct contains a MancalaBoard as well as all information needed to function as a tree-node for the MiniMax Search Algorithm.

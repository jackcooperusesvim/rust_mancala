.SILENT:
run:
	echo "This is a simple MiniMax Algorithm I wrote in Rust to play Mancala. The algorithm is a simple tree search which builds a tree (breadth-first) downward to a certain depth, finds the \"utility\" of each node at that bottom layer, and then propogates those utility values from child to parent nodes depending on whose turn it is, and what their optimal utility is. You can see in the logs how each board takes progressively longer to build, and the utilities take longer to propogate. The MancalaBoard object shows the information stored in the top node for the tree, and the following boolean and Option (just read the number) refer to whether or not the space has been completely searched, and the score difference between player1 and player2 at the end of the game assuming perfect play is followed, and the game ends at the bottommost node on the tree."

	read -p "Press Enter to run some (basic) tests" </dev/tty

	cargo test

	read -p "Press Enter to run the program" </dev/tty
	cargo run

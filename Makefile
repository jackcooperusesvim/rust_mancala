.SILENT:
run:
	less READ*


	read -p "Press Enter to run some (basic) tests" </dev/tty

	cargo test

	read -p "Press Enter to run the program" </dev/tty
	cargo run

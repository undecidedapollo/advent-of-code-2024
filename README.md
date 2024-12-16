# Advent of Code 2024

These are my solutions to the [Advent of Code 2024](https://adventofcode.com/2024) puzzles.

Some are brute force, some are clean, some are just for fun. I used Rust for all solutions to improve my understanding of the language.


You can run any solution with `cargo run --bin part1 input.txt` or `cargo run --bin part2 input.txt` respectively.
Some parts are combined together (like day 6).

## Day 6
This day took way to long, I spent lots of time working out issues such as forgetting to not spawn one where the guard is already standing. Not blocking your path from earlier. Remembering you can hit the obstacle from multiple directions (input4.txt). Handling edges of the screen. Etc. Finally got it right.

# Tic Tac Toe

This is a classic tic tac toe written in Rust with two agents.

The agent for player X uses min max.
The agent for player Y makes the first available move.

To run this game, download the code to your computer, and inside its directory,
use a terminal to execute the following command
```
cargo run
```

## Min Max algorithm

Min Max explores all the possible moves that each players can make.

This exploration takes the form of a tree. The starting root of the tree is
the current state of the board. Min max finds the next level of possibilities in the tree
by applying each possible move to the root independently. It then repeats this process for every
new board, until no more moves are available.

When the algorithm reachs a terminal game state (i.e. a board with no more moves or
a board with 3 consecutive Xs or Os), it assigns a score to the board based on who won.
In this implementation, we assign a score of 1 when X wins, -1 when O wins, and 0 for draw.

The algorithm then propoagates the score back up the tree all the way to the root. When
the algorithm is going up moves made by player X, it selects the move that yields the maximum
score, when the move is made by player O, it selects the move with the minimum score.

For more exploration of the algorithm, as well as implementations of it in other languages,
check these resources out:

1. https://www.geeksforgeeks.org/minimax-algorithm-in-game-theory-set-1-introduction/
2. https://www.youtube.com/watch?v=5y2a0Zhgq0U

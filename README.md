# chess_engine

## Sides

- white
- black

## characters

- king x 1 ♔
- queen x 1 ♕
- knight x 2 ♘
- pawn x 8 ♙
- rook x 2 ♖
- bishop x 2 ♗

## Positions

- King: 1E
- Queen: 1D
- Knight: 1B, 1G
- Pawn: 2(A-H)
- Rook: 1A, 1H
- Bishop: 1C, 1F

## Moves

### King

- can move in x one step
- can move in + one step

|     | A   | B   | C   | D   | E   | F   | G   | H   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 8   |     |     |     |     |     |     |     |     |
| 7   |     |     |     |     |     |     |     |     |
| 6   |     |     |     |     |     |     |     |     |
| 5   |     |     | x   | x   | x   |     |     |     |
| 4   |     |     | x   | ♔   | x   |     |     |     |
| 3   |     |     | x   | x   | x   |     |     |     |
| 2   |     |     |     |     |     |     |     |     |
| 1   |     |     |     |     |     |     |     |     |

### Queen

- can move in x infinitely
- can move in + infinitely

|     | A   | B   | C   | D   | E   | F   | G   | H   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 8   |     |     |     | x   |     |     |     | x   |
| 7   | x   |     |     | x   |     |     | x   |     |
| 6   |     | x   |     | x   |     | x   |     |     |
| 5   |     |     | x   | x   | x   |     |     |     |
| 4   | x   | x   | x   | ♕   | x   | x   | x   | x   |
| 3   |     |     | x   | x   | x   |     |     |     |
| 2   |     | x   |     | x   |     | x   |     |     |
| 1   | x   |     |     | x   |     |     | x   |     |

### Knight

- 2 forward, then left/right

|     | A   | B   | C   | D   | E   | F   | G   | H   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 8   |     |     |     |     |     |     |     |     |
| 7   |     |     |     |     |     |     |     |     |
| 6   |     |     | x   |     | x   |     |     |     |
| 5   |     | x   |     |     |     | x   |     |     |
| 4   |     |     |     | ♘   |     |     |     |     |
| 3   |     | x   |     |     |     | x   |     |     |
| 2   |     |     | x   |     | x   |     |     |     |
| 1   |     |     |     |     |     |     |     |     |

### Pawn

- starting may be 1,2 forward,
- after then fix 1 forward
- if there's someone on top left/right can move,

|     | A   | B   | C   | D    | E   | F    | G   | H   |
| --- | --- | --- | --- | ---- | --- | ---- | --- | --- |
| 8   |     |     | ♙\* |      |     |      |     |     |
| 7   |     |     |     | ♟︎x |     | ♟︎x | x   |     |
| 6   |     |     |     |      |     |      | ♙   |     |
| 5   |     |     | x   |      |     |      |     |     |
| 4   | x   | x   | ♙   | x    | x   | x    |     | x   |
| 3   | ♙   | x   |     | x    | x   | x    |     | x   |
| 2   |     | ♙   |     | ♙    | ♙   | ♙    |     | ♙   |
| 1   |     |     |     |      |     |      |     |     |

> - mendatory required to swap with any of following:
>   ♖, ♗, ♘, ♕

### Rook

- can move in + infinitely

|     | A   | B   | C   | D   | E   | F   | G   | H   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 8   |     |     |     | x   |     |     |     |     |
| 7   |     |     |     | x   |     |     |     |     |
| 6   |     |     |     | x   |     |     |     |     |
| 5   |     |     |     | x   |     |     |     |     |
| 4   | x   | x   | x   | ♖   | x   | x   | x   | x   |
| 3   |     |     |     | x   |     |     |     |     |
| 2   |     |     |     | x   |     |     |     |     |
| 1   |     |     |     | x   |     |     |     |     |

### Bishop

- can move in x infinitely
  | | A | B | C | D | E | F | G | H |
  | --- | --- | --- | --- | --- | --- | --- | --- | --- |
  | 8 | | | | | | | | x |
  | 7 | x | | | | | | x | |
  | 6 | | x | | | | x | | |
  | 5 | | | x | | x | | | |
  | 4 | | | | ♗ | | | | |
  | 3 | | | x | | x | | | |
  | 2 | | x | | | | x | | |
  | 1 | x | | | | | | x | |

### Pseudo API for this chess_game_engine

```txt
let game = new Game;

// start the game, player should make a move immediatly.
// `start` also starts a counter.
game.start(); // with white
game.start_with(White/Black);

// checks who's turn is this
let who = game.whose_turn(); // white/black

// show a visual representation of chess, through stdout
game.show_board();

// to select a character
let character = game.select(Pawn(1, G)); // warns if not legal

// list out possible moves for selected character.
let moves = character.possible_moves();

// move character to new position,
// in case of valid move, it spits out effect
// otherwise throws a error.
// also makes the timer stop.
let res = character.move_to(2, G); // ok/err

// can't select character, if you dont' switch side.
game.select(Pawan(2,G)); warns since turn over.

// changes the side to get select working again.
// also starts a counter.
game.change_side();

let who = game.whose_turn(); // white/black

game.accept_draw();

game.is_game_over();

game.check_winner();
```

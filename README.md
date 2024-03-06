# chess_engine

## Sides

- white
- black

## Characters

- K: King x 1 ♔
- Q: Queen x 1 ♕
- N: Knight x 2 ♘
- P: Pawn x 8 ♙
- R: Rook x 2 ♖
- B: Bishop x 2 ♗

## Initial Positions (White)

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

- 2 forward in anyone of the + direction then left/right

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

- starting 1,(2 optional) forward,
- after that always 1 forward
- if there's opponent's character on top-left / top-right can move,

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

> - mandatory required to swap with any of the following:
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

```javascript
let game = new Game();

// start the game, `start` also starts a counter.
game.start(); // with white
// The player should make a move.

game.start_with(White / Black);

// checks who's turn is this
let who = game.whose_turn(); // white/black

// show a visual representation of chess, through stdout
game.show_board();

// to select a character //if this engine doesn't provide GUI selection replace select with move
let character = game.select(1, G); // warns if not legal

// List out possible moves for selected character.
let moves = character.possible_moves();

// Move the character to a new position,
// In case of a valid move, it spits out the effect
// otherwise throws an error.
// also makes the timer stop.
let res = character.move_to(2, G); // ok/err

// can't select a character, if you don't switch side.
game.select(Pawn(2, G)); // warns since turn over.

// changes the side to get select working again.
// also starts a counter.
game.change_side();

let who = game.whose_turn(); // white/black

game.request_draw();

game.resign();

game.is_game_over();

game.check_winner();

game.can_pawn_promote();
```

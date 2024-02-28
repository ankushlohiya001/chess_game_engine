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

- can move in + infinitely

### Bishop

|     | A   | B   | C   | D   | E   | F   | G   | H   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 8   |     |     |     |     |     |     |     | x   |
| 7   | x   |     |     |     |     |     | x   |     |
| 6   |     | x   |     |     |     | x   |     |     |
| 5   |     |     | x   |     | x   |     |     |     |
| 4   |     |     |     | ♗   |     |     |     |     |
| 3   |     |     | x   |     | x   |     |     |     |
| 2   |     | x   |     |     |     | x   |     |     |
| 1   | x   |     |     |     |     |     | x   |     |

- can move in x infinitely

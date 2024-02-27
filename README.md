# chess_engine

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

|     |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |     |
|     |     | x   | x   | x   |     |     |     |
|     |     | x   | ♔   | x   |     |     |     |
|     |     | x   | x   | x   |     |     |     |     |
|     |     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |     |

### Queen

- can move in x infinitely
- can move in + infinitely

|     |     |     | x   |     |     |     | x   |
| --- | --- | --- | --- | --- | --- | --- | --- |
| x   |     |     | x   |     |     | x   |     |
|     | x   |     | x   |     | x   |     |     |
|     |     | x   | x   | x   |     |     |     |
| x   | x   | x   | ♕   | x   | x   | x   | x   |
|     |     | x   | x   | x   |     |     |     |
|     | x   |     | x   |     | x   |     |     |
| x   |     |     | x   |     |     | x   |     |

### Knight

- 2 forward, then left/right

|     |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     |     |     |     |     |     |
|     |     | x   |     | x   |     |     |     |
|     | x   |     |     |     | x   |     |     |
|     |     |     | ♘   |     |     |     |     |
|     | x   |     |     |     | x   |     |     |     |
|     |     | x   |     | x   |     |     |     |
|     |     |     |     |     |     |     |     |

### Pawn

- starting may be 1,2 forward,
- after then fix 1 forward
- if there's someone on top left/right can move,

|     |     |     |      |     |      |     |     |
| --- | --- | --- | ---- | --- | ---- | --- | --- |
|     |     |     |      |     |      |     |     |
|     |     |     |      |     |      |     |     |
|     |     | x   | ♟︎x |     | ♟︎x | x   |     |
| x   |     | ♙   |      |     |      | ♙   | x   |
| ♙   |     |     |      |     |      |     | x   |
|     | ♙   |     | ♙    | ♙   | ♙    |     | ♙   |
|     |     |     |      |     |      |     |     |

p: our
P: opponent

### Rook

|     |     |     | x   |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | x   |     |     |     |     |
|     |     |     | x   |     |     |     |     |
|     |     |     | x   |     |     |     |     |
| x   | x   | x   | ♖   | x   | x   | x   | x   |
|     |     |     | x   |     |     |     |     |     |
|     |     |     | x   |     |     |     |     |
|     |     |     | x   |     |     |     |     |

- can move in + infinitely

### Bishop

|     |     |     |     |     |     |     | x   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| x   |     |     |     |     |     | x   |     |
|     | x   |     |     |     | x   |     |     |
|     |     | x   |     | x   |     |     |     |
|     |     |     | ♗   |     |     |     |     |
|     |     | x   |     | x   |     |     |     |     |
|     | x   |     |     |     | x   |     |     |
| x   |     |     |     |     |     | x   |     |

- can move in x infinitely

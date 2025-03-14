> [!NOTE]
> This file is used to generate [the Typst Universe page](https://typst.app/universe/package/board-n-pieces). It is processed by [`/build.py`](/build.py).


# Board & Pieces

This package lets you display and customize chessboards. It supports FEN and PGN and can display different kinds of markings such as arrows. It also adds names for various chess-related symbols.


## Displaying chessboards

The main function of this package is `board`. It lets you display a specific position on a board.

```example
#board(starting-position)
```

`starting-position` is a position that is provided by the package. It represents the initial position of a chess game.

You can create a different position using the `position` function. It accepts strings representing each rank. Use upper-case letters for white pieces, and lower-case letters for black pieces. Dots and spaces correspond to empty squares.

```example
#board(position(
  "....r...",
  "........",
  "..p..PPk",
  ".p.r....",
  "pP..p.R.",
  "P.B.....",
  "..P..K..",
  "........",
))
```

Alternatively, you can use the `fen` function to create a position using [Forsyth–Edwards notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation):

```example
#board(fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 1 23"))
```

Note that you can specify only the first part of the FEN string:

```example
#board(fen("r4rk1/pp2Bpbp/1qp3p1/8/2BP2b1/Q1n2N2/P4PPP/3RK2R"))
```

Also note that positions do not need to be on a standard 8×8 board:

```example
#board(position(
  "....Q....",
  "......Q..",
  "........Q",
  "...Q.....",
  ".Q.......",
  ".......Q.",
  ".....Q...",
  "..Q......",
  "Q........",
))
```


## Manipulating positions

A position is a value that can be returned by the `position` or `fen` function. Positions can be manipulated in various ways.

### Reversing positions

A position can be inverted using the `invert-position` function. Reversing a position consists of mirroring it, and changing the color of the pieces. For example, this can be used to turn a Black to move puzzle into a White to move puzzle.

```example
// From https://chesspuzzle.net/Puzzle/928240.
#let puzzle = position(
  "kb.....r",
  ".p...ppp",
  "pQ..p...",
  ".N.....q",
  "..P..n..",
  "R.......",
  "PP...PPn",
  "....R.K.",
)
#stack(
  dir: ltr,
  spacing: 0.5cm,
  board(reverse: true, puzzle),
  board(invert-position(puzzle)),
)
```

### Applying a turn to a position

The `play` function creates an array containing the successive results of applying turns to a starting position. Turns are described by a series of turns written in [standard algebraic notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)). Those turns can be specified as an array of strings, or as a single string containing whitespace-separated moves. In particular, this can be used to generate the intermediate positions of an entire chess game.

```example
%show: pad.with(0.5cm)
The scholar's mate:
#let positions = play("e4 e5 Qh5 Nc6 Bc4 Nf6 Qxf7")
#grid(
  columns: 4,
  gutter: 0.2cm,
  ..positions.map(board.with(square-size: 0.5cm)),
)
```

You can specify an alternative starting position to the `play` function with the `starting-position` named argument. This can be used to apply a single move to a specific position.

```example
#let initial = fen("r1bqkbnr/ppp1pppp/2n5/3p4/3P4/4P3/PPP2PPP/RNBQKBNR")
#let next = play(starting-position: initial, "Bb5").last()
#stack(
  dir: ltr,
  spacing: 0.5cm,
  board(initial),
  board(next),
)
```


## Using the `pgn` function to import PGN files

Similarly to the `play` function, the `pgn` function creates an array of positions. It accepts a single argument, which is a string containing [portable game notation](https://en.wikipedia.org/wiki/Portable_Game_Notation). To read a game from a PGN file, you can use this function in combination with Typst's native [`read`](https://typst.app/docs/reference/data-loading/read/) function.

```typ
#let positions = pgn(read("game.pgn"))
```

Note that the argument to `pgn` must describe a single game. If you have a PGN file containing multiple games, you will need to split them using other means.


## Using non-standard chess pieces

The `board` function's `pieces` argument lets you specify how to display pieces by mapping each piece character to some content. You can use this feature to display non-standard chess pieces:

```example
%set text(size: 0.8cm, font: "Noto Sans Symbols 2")
#board(
  fen("g7/5g2/8/8/8/8/p6g/k1K4G"),
  pieces: (
    // We use symbols for the example.
    // In practice, you should import your own images.
    g: chess-sym.queen.black.b,
    p: chess-sym.pawn.black,
    k: chess-sym.king.black,
    K: chess-sym.king.white,
    G: chess-sym.queen.white.b,
  ),
)
```


## Customizing a chessboard

The `board` function lets you customize the appearance of the board in various ways, as illustrated in the example below.

```example
// From https://lichess.org/study/Xf1PGrM0.
#board(
  fen("3k4/7R/8/2PK4/8/8/8/6r1 b - - 0 1"),

  marked-squares: (
    "c7 c6": marks.circle(),
    "h6": marks.cross(paint: rgb("#ffca3ad0")),
  ),
  arrows: ("d8 c8", "d8 c7", "g1 g6", "h7 h6"),
  display-numbers: true,

  white-square-fill: rgb("#d2eeea"),
  black-square-fill: rgb("#567f96"),
  white-mark: marks.cross(paint: rgb("#2bcbC6")),
  black-mark: marks.cross(paint: rgb("#2bcbC6")),
  arrow-fill: rgb("#38f442df"),
  arrow-thickness: 0.25cm,

  stroke: 0.8pt + black,
)
```

Here is a list of all the available arguments:

- `marked-squares` is a list of squares to mark (e.g., `("d3", "d2", "e3")`). It can also be specified as a single string containing whitespace-separated squares (e.g., `"d3 d2 e3"`). For full customization, a dictionary can be provided, where the keys are the squares, and the values the marks to use. A set of marks is available in the `marks` module: `fill`, `circle`, and `cross`.

- `arrows` is a list of arrows to draw (e.g., `("e2 e4", "e7 e5")`).

- `reverse` is a boolean indicating whether to reverse the board, displaying it from Black's point of view. This is `false` by default, meaning the board is displayed from White's point of view.

- `display-numbers` is a boolean indicating whether ranks and files should be numbered. This is `false` by default.

- `rank-numbering` and `file-numbering` are functions describing how ranks and files should be numbered. By default they are respectively `numbering.with("1")` and `numbering.with("a")`.

- `square-size` is a length describing the size of each square. By default, this is `1cm`.

- `white-square-fill` and `black-square-fill` indicate how squares should be filled. They can be colors, gradients or patterns.

- `white-mark` and `black-mark` are the marks to use by default for the corresponding squares.

- `arrow-fill` and `arrow-thickness` describe how to draw the arrows.

- `pieces` is a dictionary containing images representing each piece. If specified, the dictionary must contain an entry for every piece kind in the displayed position. Keys are single upper-case letters for white pieces and single lower-case letters for black pieces. The default images are taken from [Wikimedia Commons](https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces). Please refer to [the section on licensing](#licensing) for information on how you can use them in your documents.

- `stroke` has the same structure as [`rect`'s `stroke` parameter](https://typst.app/docs/reference/visualize/rect/#parameters-stroke) and corresponds to the stroke to use around the board. If `display-numbers` is `true`, the numbers are displayed outside the stroke. The default value is `none`.


## Chess symbols

This package also exports chess symbols for all Unicode chess-related codepoints under the `chess-sym` submodule. Standard chess pieces are available as `chess-sym.{pawn,knight,bishop,rook,queen,king}.{white,black,neutral}`. Alternatively, you can use `stroked` and `filled` instead of, respectively, `white` and `black`. They can be rotated rightward, downward, and leftward respectively with with `.r`, `.b`, and `.l`. Chinese chess pieces are also available as `chess-sym.{soldier,cannon,chariot,horse,elephant,mandarin,general}.{red,black}`. Similarly, you can use `stroked` and `filled` as alternatives to, respectively, `red` and `black`. Note that most fonts only support black and white versions of standard pieces. To use the other symbols, you may have to use a font such as Noto Sans Symbols 2.

```example
%show: pad.with(0.5cm)
The best move in this position is #chess-sym.knight.white;c6.
```


## Licensing

The default images for chess pieces used by the `board` function come from [Wikimedia Commons](https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces). They are all licensed under the [GNU General Public License, version 2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html) by their original author: [Cburnett](https://en.wikipedia.org/wiki/User:Cburnett).

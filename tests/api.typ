#import sys.inputs.lib as bnp

#set document(date: none)

#set page(
  width: auto,
  height: auto,
  margin: 0.5cm,
)

#show "—": pagebreak()


// Tests start here.
// Tests are separated by `---`.


#bnp.board(bnp.starting-position)

---
#{
  set text(dir: rtl)
  bnp.board(
    bnp.starting-position,
  )
}

---
#bnp.board(
  bnp.starting-position,
  highlighted-squares: ("e4", "d4", "e5", "d5", "f4"),
)

---
#bnp.board(
  bnp.starting-position,
  highlighted-squares: "e4 d4 e5 d5 f4",
)

---
#bnp.board(
  bnp.starting-position,
  display-numbers: true,
)

---
#bnp.board(
  bnp.starting-position,
  display-numbers: true,
  rank-numbering: numbering.with("i"),
  file-numbering: numbering.with("*"),
)

---
#bnp.board(
  bnp.starting-position,
  display-numbers: true,
  reverse: true,
)

---
#bnp.board(
  bnp.starting-position,
  square-size: 0.5cm,
)

---
#bnp.board(bnp.position(
  "r..k...r",
  ".bp..ppp",
  ".pK---..",
  "p  pp.-.",
  "........",
  "P... -..",
  ".PPP..PP",
  "R.BK..NR",
))

---
#bnp.board(bnp.fen("b2r3r/k3qp1p/pn3np1/Npp5/3pPQ2/P1N2PPB/1PP4P/1K1RR3 w - - 0 22"))

---
#bnp.board(
  bnp.starting-position,
  stroke: stroke(paint: red, thickness: 5pt, dash: "dotted"),
)

---
#bnp.board(
  bnp.starting-position,
  display-numbers: true,
  stroke: 1pt + black,
)

---
#bnp.board(
  bnp.starting-position,
  display-numbers: true,
  stroke: (y: blue),
)

---
#bnp.board(
  bnp.starting-position,
  display-numbers: true,
  stroke: (bottom: none, rest: green),
)

---
#bnp.board(
  bnp.starting-position,
  highlighted-squares: "h5",
  arrows: ("e2 e4", "e7e5", "d1h5"),
  display-numbers: true,
)

---
#bnp.board(
  bnp.starting-position,
  arrows: (
    "c3 d4", "e3 d4", "c5 d4", "e5 d4", "d3 d4",
    "g4 g5", "f5 g5", "h5 g5", "g6 g5",
    "g1 f3", "f3 h4",
  ),
  display-numbers: true,
)

---
// From https://lichess.org/study/Xf1PGrM0.
#bnp.board(
  bnp.fen("3k4/7R/8/2PK4/8/8/8/6r1 b - - 0 1"),

  highlighted-squares: "c7 c6 h6",
  arrows: ("d8 c8", "d8 c7", "g1 g6", "h7 h6"),
  display-numbers: true,

  white-square-fill: rgb("D2EEEA"),
  black-square-fill: rgb("567F96"),
  highlighted-white-square-fill: rgb("69F7E4"),
  highlighted-black-square-fill: rgb("2BCBC6"),
  arrow-stroke: 0.2cm + rgb("38F442DF"),

  stroke: 0.8pt + black,
)

---
#bnp.board(
  bnp.fen("U"),
  pieces: (
    // From https://commons.wikimedia.org/wiki/File:Chess_Ult45.svg.
    // Knight by Cburnett <https://en.wikipedia.org/wiki/User:Cburnett>.
    // Unicorn by Francois-Pier <https://commons.wikimedia.org/wiki/User:Francois-Pier>.
    // Licensed under the GNU General Public License, version 2 <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html>.
    U: image("assets/unicorn.svg"),
  )
)

---
#set text(font: "Noto Sans Symbols 2")

#bnp.chess-sym.pawn.filled
#bnp.chess-sym.pawn.filled.r
#bnp.chess-sym.pawn.filled.b
#bnp.chess-sym.pawn.filled.l
#bnp.chess-sym.pawn.stroked
#bnp.chess-sym.pawn.stroked.r
#bnp.chess-sym.pawn.stroked.b
#bnp.chess-sym.pawn.stroked.l
#bnp.chess-sym.pawn.white
#bnp.chess-sym.pawn.white.r
#bnp.chess-sym.pawn.white.b
#bnp.chess-sym.pawn.white.l
#bnp.chess-sym.pawn.black
#bnp.chess-sym.pawn.black.r
#bnp.chess-sym.pawn.black.b
#bnp.chess-sym.pawn.black.l
#bnp.chess-sym.pawn.neutral
#bnp.chess-sym.pawn.neutral.r
#bnp.chess-sym.pawn.neutral.b
#bnp.chess-sym.pawn.neutral.l

#bnp.chess-sym.knight.filled
#bnp.chess-sym.knight.filled.r
#bnp.chess-sym.knight.filled.b
#bnp.chess-sym.knight.filled.l
#bnp.chess-sym.knight.stroked
#bnp.chess-sym.knight.stroked.r
#bnp.chess-sym.knight.stroked.b
#bnp.chess-sym.knight.stroked.l
#bnp.chess-sym.knight.white
#bnp.chess-sym.knight.white.r
#bnp.chess-sym.knight.white.b
#bnp.chess-sym.knight.white.l
#bnp.chess-sym.knight.black
#bnp.chess-sym.knight.black.r
#bnp.chess-sym.knight.black.b
#bnp.chess-sym.knight.black.l
#bnp.chess-sym.knight.neutral
#bnp.chess-sym.knight.neutral.r
#bnp.chess-sym.knight.neutral.b
#bnp.chess-sym.knight.neutral.l

#bnp.chess-sym.bishop.filled
#bnp.chess-sym.bishop.filled.r
#bnp.chess-sym.bishop.filled.b
#bnp.chess-sym.bishop.filled.l
#bnp.chess-sym.bishop.stroked
#bnp.chess-sym.bishop.stroked.r
#bnp.chess-sym.bishop.stroked.b
#bnp.chess-sym.bishop.stroked.l
#bnp.chess-sym.bishop.white
#bnp.chess-sym.bishop.white.r
#bnp.chess-sym.bishop.white.b
#bnp.chess-sym.bishop.white.l
#bnp.chess-sym.bishop.black
#bnp.chess-sym.bishop.black.r
#bnp.chess-sym.bishop.black.b
#bnp.chess-sym.bishop.black.l
#bnp.chess-sym.bishop.neutral
#bnp.chess-sym.bishop.neutral.r
#bnp.chess-sym.bishop.neutral.b
#bnp.chess-sym.bishop.neutral.l

#bnp.chess-sym.rook.filled
#bnp.chess-sym.rook.filled.r
#bnp.chess-sym.rook.filled.b
#bnp.chess-sym.rook.filled.l
#bnp.chess-sym.rook.stroked
#bnp.chess-sym.rook.stroked.r
#bnp.chess-sym.rook.stroked.b
#bnp.chess-sym.rook.stroked.l
#bnp.chess-sym.rook.white
#bnp.chess-sym.rook.white.r
#bnp.chess-sym.rook.white.b
#bnp.chess-sym.rook.white.l
#bnp.chess-sym.rook.black
#bnp.chess-sym.rook.black.r
#bnp.chess-sym.rook.black.b
#bnp.chess-sym.rook.black.l
#bnp.chess-sym.rook.neutral
#bnp.chess-sym.rook.neutral.r
#bnp.chess-sym.rook.neutral.b
#bnp.chess-sym.rook.neutral.l

#bnp.chess-sym.queen.filled
#bnp.chess-sym.queen.filled.r
#bnp.chess-sym.queen.filled.b
#bnp.chess-sym.queen.filled.l
#bnp.chess-sym.queen.stroked
#bnp.chess-sym.queen.stroked.r
#bnp.chess-sym.queen.stroked.b
#bnp.chess-sym.queen.stroked.l
#bnp.chess-sym.queen.white
#bnp.chess-sym.queen.white.r
#bnp.chess-sym.queen.white.b
#bnp.chess-sym.queen.white.l
#bnp.chess-sym.queen.black
#bnp.chess-sym.queen.black.r
#bnp.chess-sym.queen.black.b
#bnp.chess-sym.queen.black.l
#bnp.chess-sym.queen.neutral
#bnp.chess-sym.queen.neutral.r
#bnp.chess-sym.queen.neutral.b
#bnp.chess-sym.queen.neutral.l

#bnp.chess-sym.king.filled
#bnp.chess-sym.king.filled.r
#bnp.chess-sym.king.filled.b
#bnp.chess-sym.king.filled.l
#bnp.chess-sym.king.stroked
#bnp.chess-sym.king.stroked.r
#bnp.chess-sym.king.stroked.b
#bnp.chess-sym.king.stroked.l
#bnp.chess-sym.king.white
#bnp.chess-sym.king.white.r
#bnp.chess-sym.king.white.b
#bnp.chess-sym.king.white.l
#bnp.chess-sym.king.black
#bnp.chess-sym.king.black.r
#bnp.chess-sym.king.black.b
#bnp.chess-sym.king.black.l
#bnp.chess-sym.king.neutral
#bnp.chess-sym.king.neutral.r
#bnp.chess-sym.king.neutral.b
#bnp.chess-sym.king.neutral.l

#bnp.chess-sym.knight.filled.tr
#bnp.chess-sym.knight.filled.br
#bnp.chess-sym.knight.filled.bl
#bnp.chess-sym.knight.filled.tl
#bnp.chess-sym.knight.stroked.tr
#bnp.chess-sym.knight.stroked.br
#bnp.chess-sym.knight.stroked.bl
#bnp.chess-sym.knight.stroked.tl
#bnp.chess-sym.knight.white.tr
#bnp.chess-sym.knight.white.br
#bnp.chess-sym.knight.white.bl
#bnp.chess-sym.knight.white.tl
#bnp.chess-sym.knight.black.tr
#bnp.chess-sym.knight.black.br
#bnp.chess-sym.knight.black.bl
#bnp.chess-sym.knight.black.tl
#bnp.chess-sym.knight.neutral.tr
#bnp.chess-sym.knight.neutral.br
#bnp.chess-sym.knight.neutral.bl
#bnp.chess-sym.knight.neutral.tl

#bnp.chess-sym.knight.bishop.filled
#bnp.chess-sym.knight.bishop.stroked
#bnp.chess-sym.knight.bishop.white
#bnp.chess-sym.knight.bishop.black
#bnp.chess-sym.knight.rook.filled
#bnp.chess-sym.knight.rook.stroked
#bnp.chess-sym.knight.rook.white
#bnp.chess-sym.knight.rook.black
#bnp.chess-sym.knight.queen.filled
#bnp.chess-sym.knight.queen.stroked
#bnp.chess-sym.knight.queen.white
#bnp.chess-sym.knight.queen.black

#bnp.chess-sym.equihopper.filled
#bnp.chess-sym.equihopper.filled.rot
#bnp.chess-sym.equihopper.stroked
#bnp.chess-sym.equihopper.stroked.rot
#bnp.chess-sym.equihopper.white
#bnp.chess-sym.equihopper.white.rot
#bnp.chess-sym.equihopper.black
#bnp.chess-sym.equihopper.black.rot
#bnp.chess-sym.equihopper.neutral
#bnp.chess-sym.equihopper.neutral.rot

#bnp.chess-sym.soldier.filled
#bnp.chess-sym.soldier.stroked
#bnp.chess-sym.soldier.red
#bnp.chess-sym.soldier.black
#bnp.chess-sym.cannon.filled
#bnp.chess-sym.cannon.stroked
#bnp.chess-sym.cannon.red
#bnp.chess-sym.cannon.black
#bnp.chess-sym.chariot.filled
#bnp.chess-sym.chariot.stroked
#bnp.chess-sym.chariot.red
#bnp.chess-sym.chariot.black
#bnp.chess-sym.horse.filled
#bnp.chess-sym.horse.stroked
#bnp.chess-sym.horse.red
#bnp.chess-sym.horse.black
#bnp.chess-sym.elephant.filled
#bnp.chess-sym.elephant.stroked
#bnp.chess-sym.elephant.red
#bnp.chess-sym.elephant.black
#bnp.chess-sym.mandarin.filled
#bnp.chess-sym.mandarin.stroked
#bnp.chess-sym.mandarin.red
#bnp.chess-sym.mandarin.black
#bnp.chess-sym.general.filled
#bnp.chess-sym.general.stroked
#bnp.chess-sym.general.red
#bnp.chess-sym.general.black

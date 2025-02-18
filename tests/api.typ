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
  marked-squares: ("e4", "d4", "e5", "d5", "f4"),
)

---
#bnp.board(
  bnp.starting-position,
  marked-squares: "e4 d4 e5 d5 f4",
  reverse: true,
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
  marked-squares: "h5",
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

  marked-squares: "c7 c6 h6",
  arrows: ("d8 c8", "d8 c7", "g1 g6", "h7 h6"),
  display-numbers: true,

  white-square-fill: rgb("D2EEEA"),
  black-square-fill: rgb("567F96"),
  white-mark: bnp.marks.circle(paint: rgb("#2bcbc6")),
  black-mark: bnp.marks.circle(paint: rgb("#2bcbc6")),
  arrow-fill: rgb("#38f442df"),

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
  ),
)

---
#include "symbols.typ"

---
#bnp.board(
  bnp.starting-position,
  marked-squares: (
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
  ),
  white-mark: rect(fill: blue),
)

---
#bnp.board(
  bnp.starting-position,
  arrows: (
    "c3 d4", "e3 d4", "c5 d4", "e5 d4", "d3 d4",
    "g4 g5", "f5 g5", "h5 g5", "g6 g5",
    "g1 f3", "f3 h4",
  ),
  reverse: true,
)

---
#bnp.board(
  bnp.fen("8/8/8/8/8/8/8/8"),
  marked-squares: (
    a1: bnp.marks.circle(paint: blue, thickness: 0.01cm, margin: 0.2cm),
    a2: bnp.marks.cross(paint: green, thickness: 0.1cm, margin: 0.2cm),
    a3: bnp.marks.fill(bnp.marks.default-color),
    a4: bnp.marks.circle(paint: blue, thickness: 0.4cm, margin: 0cm),
  ),

  arrows: "a1 a8",
  arrow-fill: tiling(text(size: 6pt)[up]),
  arrow-thickness: 25%,
)

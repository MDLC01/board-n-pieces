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

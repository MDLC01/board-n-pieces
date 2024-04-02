#import "../target/lib.typ" as bnp

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
#set text(dir: rtl)
#bnp.board(
  bnp.starting-position,
)

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
// Hamppe vs. Meitner, 1872 (https://en.wikipedia.org/wiki/Immortal_Draw)
#let g = bnp.game("e4 e5 Nc3 Bc5 Na4 Bxf2 Kxf2 Qh4 Ke3 Qf4 Kd3 d5 Kc3 Qxe4 Kb3 Na6 a3 Qxa4 Kxa4 Nc5 Kb4 a5 Kxc5 Ne7 Bb5 Kd8 Bc6 b6 Kb5 Nxc6 Kxc6 Bb7 Kb5 Ba6 Kc6 Bb7")

#bnp.board(g.first())
#bnp.board(g.at(17))
#bnp.board(g.last())

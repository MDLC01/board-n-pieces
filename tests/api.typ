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


#bnp.display-board(bnp.starting-position)

---
#bnp.display-board(
  bnp.starting-position,
  highlighted-squares: ("e4", "d4", "e5", "d5", "f4"),
)

---
#bnp.display-board(
  bnp.starting-position,
  display-numbers: true,
)

---
#bnp.display-board(
  bnp.starting-position,
  display-numbers: true,
  rank-numbering: numbering.with("i"),
  file-numbering: numbering.with("*"),
)

---
#bnp.display-board(
  bnp.starting-position,
  display-numbers: true,
  reverse: true,
)

---
#bnp.display-board(
  bnp.starting-position,
  square-size: 0.5cm,
)

---
#bnp.display-board(bnp.position(
  "r..k...r",
  ".bp..ppp",
  ".pK.....",
  "p..pp...",
  "........",
  "P.......",
  ".PPP..PP",
  "R.BK..NR",
))

---
#bnp.display-board(bnp.fen("b2r3r/k3qp1p/pn3np1/Npp5/3pPQ2/P1N2PPB/1PP4P/1K1RR3 w - - 0 22"))

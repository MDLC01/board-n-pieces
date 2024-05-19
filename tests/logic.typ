#import sys.inputs.lib as bnp

// Test starting position.
#assert.eq(bnp.starting-position.fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")

// Test `game` function.
// Hamppe vs. Meitner, 1872 (https://en.wikipedia.org/wiki/Immortal_Draw).
#let immortal-draw = bnp.game("e4 e5 Nc3 Bc5 Na4 Bxf2 Kxf2 Qh4 Ke3 Qf4 Kd3 d5 Kc3 Qxe4 Kb3 Na6 a3 Qxa4 Kxa4 Nc5 Kb4 a5 Kxc5 Ne7 Bb5 Kd8 Bc6 b6 Kb5 Nxc6 Kxc6 Bb7 Kb5 Ba6 Kc6 Bb7")
#assert.eq(immortal-draw.first().fen, bnp.starting-position.fen)
#assert.eq(immortal-draw.at(17).fen, "r1b1k1nr/ppp2ppp/n7/3pp3/N3q3/PK6/1PPP2PP/R1BQ1BNR b kq - 0 9")
#assert.eq(immortal-draw.last().fen, "r2k3r/1bp2ppp/1pK5/p2pp3/8/P7/1PPP2PP/R1BQ2NR w - - 5 19")

// Test en-passant & castling.
// Gunnar Gundersen vs. A H Fau, 1928 (from https://www.chess.com/blog/rat_4/the-elusive-en-passant-checkmate).
#assert.eq(bnp.game("e4 e6 d4 d5 e5 c5 c3 cxd4 cxd4 Bb4+ Nc3 Nc6 Nf3 Nge7 Bd3 O-O Bxh7+ Kxh7 Ng5+ Kg6 h4 Nxd4 Qg4 f5 h5+ Kh6 Nxe6+ g5 hxg6#").last().fen, "r1bq1r2/pp2n3/4N1Pk/3pPp2/1b1n2Q1/2N5/PP3PP1/R1B1K2R b KQ - 0 15")


#let test-pgn(file-name, expected-last-position) = {
  let g = bnp.pgn(read("assets/" + file-name))
  assert.eq(g.last().fen, expected-last-position)
}
// https://lichess.org/NuxTdFcv
#test-pgn("lichess-NuxTdFcv.pgn", "8/8/1pr2Pk1/p7/P5R1/3nP1P1/1B3P1P/6K1 b - - 2 50")
// https://lichess.org/4cCk7Gi5
#test-pgn("lichess-4cCk7Gi5.pgn", "5r2/5k1p/1p2p3/p1p1P3/5P1b/1P2P2q/PB3RR1/5K2 w - - 1 44")

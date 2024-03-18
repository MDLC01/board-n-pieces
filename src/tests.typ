#import "lib.typ": *


#let test-abi() = {
  import "abi.typ": *

  let test(value, to-bytes, from-bytes) = {
    let (remainder, new-value) = from-bytes(to-bytes(value))
    assert.eq(remainder.len(), 0)
    assert.eq(new-value, value)
  }

  test(0, int-to-bytes, int-from-bytes)
  test(12, int-to-bytes, int-from-bytes)
  test(1738, int-to-bytes, int-from-bytes)
  test(727262799, int-to-bytes, int-from-bytes)

  test(none, option-to-bytes.with(int-to-bytes), option-from-bytes.with(int-from-bytes))
  test(26379, option-to-bytes.with(int-to-bytes), option-from-bytes.with(int-from-bytes))

  test("a", file-to-bytes, file-from-bytes)
  test("b", file-to-bytes, file-from-bytes)
  test("c", file-to-bytes, file-from-bytes)
  test("d", file-to-bytes, file-from-bytes)
  test("e", file-to-bytes, file-from-bytes)
  test("f", file-to-bytes, file-from-bytes)
  test("g", file-to-bytes, file-from-bytes)
  test("h", file-to-bytes, file-from-bytes)

  test("1", rank-to-bytes, rank-from-bytes)
  test("2", rank-to-bytes, rank-from-bytes)
  test("3", rank-to-bytes, rank-from-bytes)
  test("4", rank-to-bytes, rank-from-bytes)
  test("5", rank-to-bytes, rank-from-bytes)
  test("6", rank-to-bytes, rank-from-bytes)
  test("7", rank-to-bytes, rank-from-bytes)
  test("8", rank-to-bytes, rank-from-bytes)

  test("a1", square-to-bytes, square-from-bytes)
  test("e4", square-to-bytes, square-from-bytes)
  test("d8", square-to-bytes, square-from-bytes)
  test("h3", square-to-bytes, square-from-bytes)

  test("w", color-to-bytes, color-from-bytes)
  test("b", color-to-bytes, color-from-bytes)

  test("P", piece-kind-to-bytes, piece-kind-from-bytes)
  test("N", piece-kind-to-bytes, piece-kind-from-bytes)
  test("B", piece-kind-to-bytes, piece-kind-from-bytes)
  test("R", piece-kind-to-bytes, piece-kind-from-bytes)
  test("Q", piece-kind-to-bytes, piece-kind-from-bytes)
  test("K", piece-kind-to-bytes, piece-kind-from-bytes)

  test("p", piece-to-bytes, piece-from-bytes)
  test("n", piece-to-bytes, piece-from-bytes)
  test("b", piece-to-bytes, piece-from-bytes)
  test("r", piece-to-bytes, piece-from-bytes)
  test("q", piece-to-bytes, piece-from-bytes)
  test("k", piece-to-bytes, piece-from-bytes)
  test("P", piece-to-bytes, piece-from-bytes)
  test("N", piece-to-bytes, piece-from-bytes)
  test("B", piece-to-bytes, piece-from-bytes)
  test("R", piece-to-bytes, piece-from-bytes)
  test("Q", piece-to-bytes, piece-from-bytes)
  test("K", piece-to-bytes, piece-from-bytes)

  test(none, square-content-to-bytes, square-content-from-bytes)
  test("p", square-content-to-bytes, square-content-from-bytes)
  test("n", square-content-to-bytes, square-content-from-bytes)
  test("b", square-content-to-bytes, square-content-from-bytes)
  test("r", square-content-to-bytes, square-content-from-bytes)
  test("q", square-content-to-bytes, square-content-from-bytes)
  test("k", square-content-to-bytes, square-content-from-bytes)
  test("P", square-content-to-bytes, square-content-from-bytes)
  test("N", square-content-to-bytes, square-content-from-bytes)
  test("B", square-content-to-bytes, square-content-from-bytes)
  test("R", square-content-to-bytes, square-content-from-bytes)
  test("Q", square-content-to-bytes, square-content-from-bytes)
  test("K", square-content-to-bytes, square-content-from-bytes)

  test(starting-position.board, board-to-bytes, board-from-bytes)

  test((white-king-side: true, white-queen-side: false, black-king-side: false, black-queen-side: true), castling-availabilities-to-bytes, castling-availabilities-from-bytes)

  test(starting-position, position-to-bytes, position-from-bytes)
}

#let functions = plugin("plugin.wasm")

#let invert-position(position) = {
  (
    type: "board-n-pieces:fen",
    fen: str(functions.invert_position(bytes(position.fen))),
  )
}

#let replay-game(starting-position, turns) = {
  let game = functions.replay_game(
    bytes(starting-position.fen),
    turns.map(bytes).join(bytes((0, )))
  )
  array(game).split(0).map(position => (
    type: "board-n-pieces:fen",
    fen: str(bytes(position))
  ))
}

#let game-from-pgn(pgn) = {
  let game = functions.game_from_pgn(
    bytes(pgn),
  )
  array(game).split(0).map(position => (
    type: "board-n-pieces:fen",
    fen: str(bytes(position))
  ))
}

/// Converts a `board-n-pieces:fen-position` to a `board-n-pieces:position`.
/// For positions, this is the identity function.
#let resolve-position(position) = {
  let message = "expected a position (hint: you can construct a position with the `position` function)"

  assert.eq(type(position), dictionary, message: message)

  if position.type == "board-n-pieces:position" {
    return position
  }

  if position.type == "board-n-pieces:fen" {
    // A `fen` object contains a `fen` entry, which is a full fen string.
    let parts = position.fen.split(" ")
    return (
      type: "board-n-pieces:position",
      fen: position.fen,
      board: parts.at(0)
        .split("/")
        .rev()
        .map(fen-rank => {
          ()
          for s in fen-rank {
            if "0".to-unicode() <= s.to-unicode() and s.to-unicode() <= "9".to-unicode() {
              (none, ) * int(s)
            } else {
              (s, )
            }
          }
        }),
      active: parts.at(1),
      castling-availabilities: (
        white-king-side: "K" in parts.at(2),
        white-queen-side: "Q" in parts.at(2),
        black-king-side: "k" in parts.at(2),
        black-queen-side: "q" in parts.at(2),
      ),
      en-passant-target-square: if parts.at(3) != "-" { parts.at(3) },
      halfmove: int(parts.at(4)),
      fullmove: int(parts.at(5)),
    )
  }

  panic(message)
}

/// Returns the index of a file.
#let file-index(f) = f.to-unicode() - "a".to-unicode()

/// Returns the index of a rank.
#let rank-index(r) = int(r) - 1

/// Returns the coordinates of a square given a square name.
#let square-coordinates(s) = {
  let (f, r) = s.clusters()
  (file-index(f), rank-index(r))
}

/// Returns the name of a square given its coordinates.
#let square-name(s) = {
  let (f, r) = s
  str.from-unicode(f + "a".to-unicode()) + str(r + 1)
}

#let stroke-sides(arg) = {
  let sides = rect(stroke: arg).stroke

  if type(sides) != dictionary {
    sides = (
      left: sides,
      top: sides,
      right: sides,
      bottom: sides,
    )
  }

  (
    left: none,
    top: none,
    right: none,
    bottom: none,
    ..sides,
  )
}

// Determine whether a move is a knight move.
//
// Coordinates are (file, rank) pairs, 0–7 integers.
// Example call: is-knight-move((1,2), (2,4))
#let is-knight-move(move) = {
  let (c1, c2) = move
  let (f1, r1) = c1
  let (f2, r2) = c2
  let dx = f1 - f2
  let dy = r1 - r2
  // Knight moves satisfy dx² + dy² = 5
  dx*dx + dy*dy == 5
}

// Draws a knight move arrow with an L-shaped path from start to end coordinates.
#let draw-knight-move-arrow(arrow, reverse, arrow-thickness, square-size, width, arrow-base-offset, arrow-fill, height) = {
  let ((start-file, start-rank), (end-file, end-rank)) = arrow
  if reverse {
    start-file = width - start-file - 1
    start-rank = height - start-rank - 1
    end-file = width - end-file - 1
    end-rank = height - end-rank - 1
  }

  let dx = end-file - start-file
  let dy = end-rank - start-rank
  
  // Determine the corner point for the L-shape
  // For knight moves, we choose the path that goes in the direction of the larger displacement first
  let corner-file = start-file
  let corner-rank = start-rank
  
  if calc.abs(dx) > calc.abs(dy) {
    // Move horizontally first, then vertically
    corner-file = end-file
    corner-rank = start-rank
  } else {
    // Move vertically first, then horizontally
    corner-file = start-file
    corner-rank = end-rank
  }

  let head-thickness = 2 * arrow-thickness
  let head-length = 1.5 * arrow-thickness
  let tip = square-size / 6

  // Calculate angles using the same coordinate system as straight arrows
  let first-angle = calc.atan2(corner-file - start-file, start-rank - corner-rank)
  let second-angle = calc.atan2(end-file - corner-file, corner-rank - end-rank)
  
  // Calculate positions
  let start-x = (start-file - width + 1) * square-size
  let start-y = -start-rank * square-size
  let corner-x = (corner-file - width + 1) * square-size
  let corner-y = -corner-rank * square-size
  let end-x = (end-file - width + 1) * square-size
  let end-y = -end-rank * square-size
  
  // Apply base offset using the same pattern as straight arrows
  let tail-x = start-x + calc.cos(first-angle) * arrow-base-offset
  let tail-y = start-y + calc.sin(first-angle) * arrow-base-offset

  return {
    // Arrows are all placed in the bottom right square.
    show: place.with(center + horizon)
    show: place

    curve(
      fill: arrow-fill,
      
      // First segment - start to corner (using straight arrow pattern)
      curve.move((
        tail-x + (calc.sin(first-angle) - calc.cos(first-angle)) * arrow-thickness / 2,
        tail-y + (-calc.cos(first-angle) - calc.sin(first-angle)) * arrow-thickness / 2,
      )),
      curve.line((
        tail-x + (-calc.sin(first-angle) - calc.cos(first-angle)) * arrow-thickness / 2,
        tail-y + (calc.cos(first-angle) - calc.sin(first-angle)) * arrow-thickness / 2,
      )),
      curve.line((
        corner-x + (-calc.sin(first-angle) - calc.cos(first-angle)) * arrow-thickness / 2,
        corner-y + (calc.cos(first-angle) - calc.sin(first-angle)) * arrow-thickness / 2,
      )),
      
      // Corner transition to second segment
      curve.line((
        corner-x + (-calc.sin(second-angle) - calc.cos(second-angle)) * arrow-thickness / 2,
        corner-y + (calc.cos(second-angle) - calc.sin(second-angle)) * arrow-thickness / 2,
      )),
      
      // Second segment to arrow head
      curve.line((
        end-x - calc.sin(second-angle) * arrow-thickness / 2 - calc.cos(second-angle) * (head-length + tip),
        end-y + calc.cos(second-angle) * arrow-thickness / 2 - calc.sin(second-angle) * (head-length + tip),
      )),
      
      // Arrow head (using straight arrow pattern)
      curve.line((
        end-x - calc.sin(second-angle) * head-thickness / 2 - calc.cos(second-angle) * (head-length + tip),
        end-y + calc.cos(second-angle) * head-thickness / 2 - calc.sin(second-angle) * (head-length + tip),
      )),
      curve.line((
        end-x - calc.cos(second-angle) * tip,
        end-y - calc.sin(second-angle) * tip,
      )),
      curve.line((
        end-x + calc.sin(second-angle) * head-thickness / 2 - calc.cos(second-angle) * (head-length + tip),
        end-y - calc.cos(second-angle) * head-thickness / 2 - calc.sin(second-angle) * (head-length + tip),
      )),
      
      // Back along second segment
      curve.line((
        end-x + calc.sin(second-angle) * arrow-thickness / 2 - calc.cos(second-angle) * (head-length + tip),
        end-y - calc.cos(second-angle) * arrow-thickness / 2 - calc.sin(second-angle) * (head-length + tip),
      )),
      curve.line((
        corner-x + (calc.sin(second-angle) - calc.cos(second-angle)) * arrow-thickness / 2,
        corner-y + (-calc.cos(second-angle) - calc.sin(second-angle)) * arrow-thickness / 2,
      )),
      
      // Corner transition back to first segment
      curve.line((
        corner-x + (calc.sin(first-angle) - calc.cos(first-angle)) * arrow-thickness / 2,
        corner-y + (-calc.cos(first-angle) - calc.sin(first-angle)) * arrow-thickness / 2,
      )),
      
      curve.close(),
    )
  }
}

// Draws a straight arrow from start to end coordinates.
#let draw-straight-arrow(arrow, reverse, arrow-thickness, square-size, width, arrow-base-offset, arrow-fill, height) = {
  let ((start-file, start-rank), (end-file, end-rank)) = arrow
  if reverse {
    start-file = width - start-file - 1
    start-rank = height - start-rank - 1
    end-file = width - end-file - 1
    end-rank = height - end-rank - 1
  }

  let angle = calc.atan2(end-file - start-file, start-rank - end-rank)
  let head-thickness = 2 * arrow-thickness
  let head-length = 1.5 * arrow-thickness
  let tip = square-size / 6
  let tail-x = (start-file - width + 1) * square-size + calc.cos(angle) * arrow-base-offset
  let tail-y = -start-rank * square-size + calc.sin(angle) * arrow-base-offset

  return {
    // Arrows are all placed in the bottom right square.
    show: place.with(center + horizon)
    show: place

    curve(
      fill: arrow-fill,
      // Base of the arrow.
      curve.move((
        tail-x + (calc.sin(angle) - calc.cos(angle)) * arrow-thickness / 2,
        tail-y + (-calc.cos(angle) - calc.sin(angle)) * arrow-thickness / 2,
      )),
      curve.line((
        tail-x + (-calc.sin(angle) - calc.cos(angle)) * arrow-thickness / 2,
        tail-y + (calc.cos(angle) - calc.sin(angle)) * arrow-thickness / 2,
      )),
      // Right before the arrow head.
      curve.line((
        (end-file - width + 1) * square-size
          - calc.sin(angle) * arrow-thickness / 2
          - calc.cos(angle) * (head-length + tip),
        -end-rank * square-size
          + calc.cos(angle) * arrow-thickness / 2
          - calc.sin(angle) * (head-length + tip),
      )),
      // Arrow head.
      curve.line((
        (end-file - width + 1) * square-size
          - calc.sin(angle) * head-thickness / 2
          - calc.cos(angle) * (head-length + tip),
        -end-rank * square-size
          + calc.cos(angle) * head-thickness / 2
          - calc.sin(angle) * (head-length + tip),
      )),
      curve.line((
        (end-file - width + 1) * square-size
          - calc.cos(angle) * tip,
        -end-rank * square-size
          - calc.sin(angle) * tip,
      )),
      curve.line((
        (end-file - width + 1) * square-size
          + calc.sin(angle) * head-thickness / 2
          - calc.cos(angle) * (head-length + tip),
        -end-rank * square-size
          - calc.cos(angle) * head-thickness / 2
          - calc.sin(angle) * (head-length + tip),
      )),
      // Right after the arrow head.
      curve.line((
        (end-file - width + 1) * square-size
          + calc.sin(angle) * arrow-thickness / 2
          - calc.cos(angle) * (head-length + tip),
        -end-rank * square-size
          - calc.cos(angle) * arrow-thickness / 2
          - calc.sin(angle) * (head-length + tip),
      )),
      curve.close(),
    )
  }
}

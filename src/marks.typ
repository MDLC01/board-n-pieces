/// The color used by default for marks.
#let default-color = rgb("#ff4136a5")

/// Fills the entire square background.
#let fill(fill) = {
  rect(width: 100%, height: 100%, fill: fill, stroke: none)
}

/// Marks a square with a circle.
#let circle(paint: default-color, thickness: 0.15cm, margin: 0.05cm) = {
  std.circle(
    width: 100% - thickness - 2 * margin,
    stroke: paint + thickness,
  )
}

/// Marks a square with a cross.
#let cross(paint: default-color, thickness: 0.15cm, margin: 0.05cm) = {
  set align(top + left)

  let offset = thickness / calc.sqrt(8)
  let start = 0% + margin + offset
  let end = 100% - margin - offset

  std.curve(
    stroke: paint + thickness,
    curve.move((50%, 50%)),
    curve.line((start, start)),
    curve.close(),
    curve.move((50%, 50%)),
    curve.line((start, end)),
    curve.close(),
    curve.move((50%, 50%)),
    curve.line((end, end)),
    curve.close(),
    curve.move((50%, 50%)),
    curve.line((end, start)),
  )
}

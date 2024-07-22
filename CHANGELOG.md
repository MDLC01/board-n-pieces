# Changelog

## Version 0.5.0

- Add symbols for all Unicode chess-related codepoints.

- Change the signature of the `board` function.
    - Rename argument `highlighted-squares` to `marked-squares`.
    - Remove arguments `highlighted-white-square-fill` and `highlighted-black-square-fill`.
    - Add argument `marking-color`, together with `marked-white-square-background` and `marked-black-square-background`.
    - Support passing a length as `arrow-stroke`.

- Fix arrows not being displayed properly on reversed boards.

## Version 0.4.0

- Add the ability to draw arrows in `board`.

## Version 0.3.0

- Detect moves that put the king in check as illegal, improving SAN support.

- Add `stroke` argument to the `board` function.

- Rename `{highlighted-,}{white,black}-square-color` arguments to the `board` function to `{highlighted-,}{white,black}-square-fill`.

## Version 0.2.0

- Allow using dashes for empty squares in `position` function.

- Allow passing highlighted squares as a single string of whitespace-separated squares.

- Describe entire games using algebraic notation with the `game` function.

- Initial PGN support through the `pgn` function.

## Version 0.1.0

- Display a chess position on a chessboard with the `board` function.

- Get the starting position with `starting-position`.

- Use chess-related symbols with the `chess-sym` module.

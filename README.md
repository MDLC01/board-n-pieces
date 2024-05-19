# Board & Pieces

This is a [Typst](https://github.com/typst/typst) package to display chessboards. It is available on [Typst Universe](https://typst.app/universe/package/board-n-pieces).


## Project structure

This project consists of a Typst library and a plugin written in Rust. Sources for the library are under [`src/`](src/), and sources for the plugin are in [`plugin/`](plugin/).

This project can be built into a proper Typst package and tested by [`build.py`](build.py). This build script was made for Python 3.12, and does not require anything outside of the standard library.

The [`tests/`](tests/) directory contains a small test suite.


## Usage

For more information on how to use this package, take a look at the rendered README on [Typst Universe](https://typst.app/universe/package/board-n-pieces).


## License

The contents of this repository are licensed under the [MIT License](LICENSE), with the exception of the chess pieces under [`src/assets/`](src/assets/). Those are licensed under the [GNU General Public License, version 2](src/assets/LICENSE). They were initially published on Wikimedia by [Cburnett](https://en.wikipedia.org/wiki/User:Cburnett), and later modified by other contributors.

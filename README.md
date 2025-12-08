# Board & Pieces

This is a [Typst](https://github.com/typst/typst) package to display chessboards. It is available on [Typst Universe](https://typst.app/universe/package/board-n-pieces).


## Project structure

This project consists of a Typst library and a plugin written in Rust. Sources for the library are under [`src/`](src/), and sources for the plugin are in [`plugin/`](plugin/).

This project can be built into a proper Typst package and tested by [`build.py`](build.py). This build script was made for Python 3.12, and does not require anything outside of the standard library.

The [`tests/`](tests/) directory contains a small test suite.


## Usage

For more information on how to use this package, take a look at the rendered README on [Typst Universe](https://typst.app/universe/package/board-n-pieces).


## Contributing

Contributions to this project are welcome. Unless you are making small changes to the Typst library, you will probably need the following tools:
- The latest version of the [Typst compiler](https://typst.app/open-source/#download)
    - Another option is to use the [Tinymist](https://myriad-dreamin.github.io/tinymist/) extension for VSCode or VSCodium, which bundles a Typst compiler.
    - Note that using the web app to contribute to Board & Pieces is not possible.
- A Rust toolchain including Cargo.
    - The simplest option is to install [Rustup](https://rust-lang.org/tools/install/#rustup).
- [Python](https://www.python.org/downloads/) 3.12 or more (the latest non-pre-release version is probably the best).

Additionally, you will need to install [Git](https://git-scm.com/install/) and learn how to fork a GitHub repository and create a pull request.

To build the package, simply run the `build.py` script with Python. On most systems, this can be achieved by executing the command `./build.py`. On Windows, you will need to run `python build.py` instead.

The build script will create a `target` directory that contains the package including the compiled plugin. You should be able to import this version of the package by creating a file at this project's root directory and using

```typ
#import "target/lib.typ" as board-n-pieces
```

The build script will also run the test suite. Some tests fill print a message in the console when they fail. Others will simply modify references images, which is detected by Git. It is up to you to determine whether the changed images are intentional or not.

### Use of generative artificial intelligence

Please limit the use of generative artificial intelligence to the bare minimum (often, this means not using it) when contributing. Reviewing AI-generated pull requests is unfulfilling as the interaction with the author brings no additional value compared to writing the code myself. Please also write your pull request descriptions yourself: the goal is to explain why you made those implementation choices, which no AI model can do unless they have access to your brain, which seems scary.


## License

Unless otherwise stated, the contents of this repository are licensed under the [MIT License](LICENSE), with the exception of the chess pieces under [`src/assets/`](src/assets/). Those are licensed under the [GNU General Public License, version 2](src/assets/LICENSE). They were initially published on Wikimedia by [Cburnett](https://en.wikipedia.org/wiki/User:Cburnett), and later modified by other contributors.

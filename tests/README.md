# Tests

This directory contains a small test suite for Board & Pieces.


## Run the tests

To run the tests, first build the package, and then run the following command within the project's root directory. Tests pass if the compilation succeeds and Git does not detect any change in the images under [`refs/`](refs/).

```shell
typst compile ./tests/tests.typ './tests/refs/test-{n}.png' --root .
```


## Structure

Tests for the package's public interface are stored in [`api.typ`](api.typ). Other tests are located in [`tests.typ`](tests.typ).

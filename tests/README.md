# Tests

This directory contains a small test suite for Board & Pieces.


## Run the tests

The tests are ran automatically when building using the build script. Tests pass if the compilation succeeds and Git does not detect any change in the images under [`refs/`](refs/).


## Structure

All tests are included in [`tests.typ`](tests.typ). They are split into different files: tests for the package's public interface are stored in [`api.typ`](api.typ), while tests for the logic (mainly the PGN parser) are located in [`logic.typ`](logic.typ).

#!/usr/bin/env python3


import os
import shutil
import subprocess

from pathlib import Path


TARGET_DIRECTORY = Path('target/')


def read_exclude_file(path: Path) -> list[str]:
    with open(path) as f:
        lines = [line.strip() for line in f.read().splitlines()]
    return [os.path.normcase(entry) for entry in lines if entry and not entry.startswith('#')]


def clone_sources():
    LIBRARY_DIRECTORY = Path('src/')
    EXCLUDE_FILE = LIBRARY_DIRECTORY.joinpath('.exclude')

    if TARGET_DIRECTORY.exists():
        shutil.rmtree(TARGET_DIRECTORY)
    TARGET_DIRECTORY.mkdir(parents=True)

    excluded_files = read_exclude_file(EXCLUDE_FILE)
    excluded_files.append(os.path.normcase(EXCLUDE_FILE.relative_to(LIBRARY_DIRECTORY)))

    for path, directories, files in os.walk(LIBRARY_DIRECTORY):
        for file in files:
            file_name = Path(path).relative_to(LIBRARY_DIRECTORY).joinpath(file)
            if os.path.normcase(file_name) not in excluded_files:
                source = LIBRARY_DIRECTORY.joinpath(file_name)
                destination = TARGET_DIRECTORY.joinpath(file_name)
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source, destination)


def build_plugin():
    BUILD_TARGET = 'wasm32-unknown-unknown'
    PLUGIN_DIRECTORY = Path('plugin/')
    PLUGIN_PATH = PLUGIN_DIRECTORY.joinpath('target', BUILD_TARGET, 'release', 'board_n_pieces_plugin.wasm')
    PLUGIN_NAME = 'plugin.wasm'

    subprocess.call(['cargo', 'build', '--release', '--target', BUILD_TARGET], cwd=PLUGIN_DIRECTORY)

    shutil.copyfile(PLUGIN_PATH, TARGET_DIRECTORY.joinpath(PLUGIN_NAME))


def main():
    clone_sources()
    build_plugin()


if __name__ == '__main__':
    main()

#!/usr/bin/env python3


import os
import shutil
import subprocess

from pathlib import Path


LICENSE = Path('LICENSE')
TARGET_DIRECTORY = Path('target/')


def preprocess_readme(content: str) -> str:
    EXAMPLES_DIRECTORY = TARGET_DIRECTORY.joinpath('examples')

    examples = []
    new_lines = []
    example = []
    is_example = False
    for line in content.splitlines():
        new_lines.append(line)
        if is_example and line.startswith('```'):
            is_example = False
            examples.append('\n'.join(example))
            new_lines.append('')
            new_lines.append(f'![image](examples/example-{len(examples)}.png)')
            new_lines.append('')
        elif is_example:
            example.append(line)
        elif line.startswith('```example'):
            is_example = True
            example = []

    example_source = [
        '#import "src/lib.typ": *;',
        '#set page(width: auto, height: auto, margin: 0cm);',
    ]
    for example in examples:
        example_source.append(f'#page[{example}];')

    EXAMPLES_DIRECTORY.mkdir(parents=True)
    subprocess.run(
        ['typst', 'compile', '-', str(EXAMPLES_DIRECTORY.joinpath('example-{n}.png'))],
        input='\n'.join(example_source),
        encoding='utf-8',
        check=True,
    )

    return '\n'.join(new_lines)


def read_exclude_file(path: Path) -> list[str]:
    with open(path) as f:
        lines = [line.strip() for line in f.read().splitlines()]
    return [os.path.normcase(entry) for entry in lines if entry and not entry.startswith('#')]


def clone_sources():
    LIBRARY_DIRECTORY = Path('src/')
    EXCLUDE_FILE = LIBRARY_DIRECTORY.joinpath('.exclude')
    README = Path('README.md')

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
                if os.path.normcase(file_name) == os.path.normcase(README):
                    with open(source) as f:
                        readme = f.read()
                    processed_readme = preprocess_readme(readme)
                    with open(destination, 'w') as f:
                        f.write(processed_readme)
                else:
                    shutil.copyfile(source, destination)
    shutil.copyfile(LICENSE, TARGET_DIRECTORY.joinpath('LICENSE'))


def build_plugin():
    BUILD_TARGET = 'wasm32-unknown-unknown'
    PLUGIN_DIRECTORY = Path('plugin/')
    PLUGIN_PATH = PLUGIN_DIRECTORY.joinpath('target', BUILD_TARGET, 'release', 'board_n_pieces_plugin.wasm')
    PLUGIN_NAME = 'plugin.wasm'

    subprocess.run(
        ['cargo', 'build', '--release', '--target', BUILD_TARGET, '--quiet'],
        cwd=PLUGIN_DIRECTORY,
        check=True,
    )

    shutil.copyfile(PLUGIN_PATH, TARGET_DIRECTORY.joinpath(PLUGIN_NAME))


def main():
    clone_sources()
    build_plugin()


if __name__ == '__main__':
    try:
        main()
    except subprocess.CalledProcessError:
        exit(1)

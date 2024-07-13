#!/usr/bin/env python3


import os
import shutil
import subprocess
import sys

from pathlib import Path


LIBRARY_DIR = Path('src/')
PLUGIN_DIR = Path('plugin/')
TARGET_DIR = Path('target/')
TEST_DIR = Path('tests/')
LICENSE = Path('LICENSE')
CHANGELOG = Path('CHANGELOG.md')
README = 'README.md'


def delete_directory_content(directory):
    if directory.exists():
        shutil.rmtree(directory)
    directory.mkdir(parents=True)


def read_exclude_file(path: Path) -> list[str]:
    with open(path) as f:
        lines = [line.strip() for line in f.read().splitlines()]
    return [os.path.normcase(entry) for entry in lines if entry and not entry.startswith('#')]


def copy_library():
    print('Copying library...')

    README_PATH = os.path.normcase(LIBRARY_DIR.joinpath(README))

    for path, directories, files in os.walk(LIBRARY_DIR):
        for file in files:
            file_name = Path(path).relative_to(LIBRARY_DIR).joinpath(file)
            if os.path.normcase(file_name) != README_PATH:
                source = LIBRARY_DIR.joinpath(file_name)
                destination = TARGET_DIR.joinpath(file_name)
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source, destination)
    shutil.copyfile(LICENSE, TARGET_DIR.joinpath('LICENSE'))


def build_plugin():
    print('Building plugin...')

    BUILD_TARGET = 'wasm32-unknown-unknown'
    PLUGIN_PATH = PLUGIN_DIR.joinpath('target', BUILD_TARGET, 'release', 'board_n_pieces_plugin.wasm')
    PLUGIN_NAME = 'plugin.wasm'

    subprocess.run(
        ['cargo', 'build', '--release', '--target', BUILD_TARGET, '--quiet'],
        cwd=PLUGIN_DIR,
        check=True,
    )

    shutil.copyfile(PLUGIN_PATH, TARGET_DIR.joinpath(PLUGIN_NAME))


def build_readme():
    print('Building README...')

    EXAMPLES_DIR = Path('examples/')

    def example_path(n):
        return EXAMPLES_DIR.joinpath(f'example-{n}.svg')

    final_lines = []

    with open(LIBRARY_DIR.joinpath(README)) as f:
        initial_readme = f.read()

    # Build examples

    example_source = [
        '#import "lib.typ": *;',
        '#set page(width: auto, height: auto, margin: 0cm, fill: rgb("#fdfdfd"));',
    ]
    example_count = 0
    example = []
    is_example = False
    is_start = True
    for line in initial_readme.splitlines():
        # Skip initial note.
        if is_start:
            if line.startswith('>') or line.strip() == '':
                continue
            is_start = False
        # Handle examples.
        if is_example and line.startswith('```'):
            is_example = False
            example_count += 1
            example_source.append(f'#page[\n{'\n'.join(example)}\n];')
            final_lines.append(line)
            final_lines.append('')
            final_lines.append(f'![image]({example_path(example_count).as_posix()})')
        elif is_example:
            if line.startswith('%'):
                example.append('#' + line[1:])
            else:
                final_lines.append(line)
                example.append(line)
        elif line.startswith('```example'):
            final_lines.append('```typ')
            is_example = True
            example = []
        else:
            final_lines.append(line)

    TARGET_DIR.joinpath(EXAMPLES_DIR).mkdir(parents=True)
    subprocess.run(
        ['typst', 'compile', '-', str(example_path('{n}'))],
        input='\n'.join(example_source),
        encoding='utf-8',
        cwd=TARGET_DIR,
        check=True,
    )

    # Add changelog

    with open(CHANGELOG) as f:
        initial_changelog = f.read()

    final_lines.append('')
    final_lines.append('')
    for line in initial_changelog.splitlines():
        if line.startswith('#'):
            final_lines.append('#' + line)
        else:
            final_lines.append(line)

    # Write README

    with open(TARGET_DIR.joinpath(README), 'w') as f:
        f.write('\n'.join(final_lines))


def test():
    print('Running tests...')

    LIB_ROOT = TARGET_DIR.joinpath('lib.typ')
    REF_DIR = TEST_DIR.joinpath('refs/')

    delete_directory_content(REF_DIR)

    subprocess.run(
        [
            'typst', 'compile',
            str(TEST_DIR.joinpath('tests.typ')),
            str(REF_DIR.joinpath('test-{n}.png')),
            '--input', f'lib={os.path.relpath(LIB_ROOT, TEST_DIR)}',
            '--root', '.',
        ],
        check=True,
    )


def main():
    delete_directory_content(TARGET_DIR)
    copy_library()
    build_plugin()
    build_readme()
    test()


if __name__ == '__main__':
    try:
        main()
    except subprocess.CalledProcessError:
        exit(1)

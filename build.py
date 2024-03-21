#!/usr/bin/env python3


import os
import shutil
import subprocess

from pathlib import Path


LIBRARY_DIR = Path('src/')
PLUGIN_DIR = Path('plugin/')
TARGET_DIR = Path('target/')
LICENSE = Path('LICENSE')
CHANGELOG = Path('CHANGELOG.md')
README = 'README.md'


def read_exclude_file(path: Path) -> list[str]:
    with open(path) as f:
        lines = [line.strip() for line in f.read().splitlines()]
    return [os.path.normcase(entry) for entry in lines if entry and not entry.startswith('#')]


def copy_library():
    print('Copying library...')

    EXCLUDE_FILE = LIBRARY_DIR.joinpath('.exclude')

    excluded_files = read_exclude_file(EXCLUDE_FILE)
    excluded_files.append(os.path.normcase(EXCLUDE_FILE.relative_to(LIBRARY_DIR)))
    excluded_files.append(os.path.normcase(LIBRARY_DIR.joinpath(README)))

    for path, directories, files in os.walk(LIBRARY_DIR):
        for file in files:
            file_name = Path(path).relative_to(LIBRARY_DIR).joinpath(file)
            if os.path.normcase(file_name) not in excluded_files:
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

    EXAMPLES_DIR = Path('examples')

    final_lines = []

    with open(LIBRARY_DIR.joinpath(README)) as f:
        initial_readme = f.read()

    # Build examples

    examples = []
    example = []
    is_example = False
    for line in initial_readme.splitlines():
        if is_example and line.startswith('```'):
            is_example = False
            examples.append('\n'.join(example))
            final_lines.append(line)
            final_lines.append('')
            final_lines.append(f'![image](examples/example-{len(examples)}.svg)')
            final_lines.append('')
        elif is_example:
            final_lines.append(line)
            example.append(line)
        elif line.startswith('```example'):
            final_lines.append('```typ')
            is_example = True
            example = []
        else:
            final_lines.append(line)

    example_source = [
        f'#import "lib.typ": *;',
        '#set page(width: auto, height: auto, margin: 0cm);',
    ]
    for example in examples:
        example_source.append(f'#page[{example}];')

    TARGET_DIR.joinpath(EXAMPLES_DIR).mkdir(parents=True)
    subprocess.run(
        ['typst', 'compile', '-', str(EXAMPLES_DIR.joinpath('example-{n}.svg'))],
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


def main():
    if TARGET_DIR.exists():
        shutil.rmtree(TARGET_DIR)
    TARGET_DIR.mkdir(parents=True)

    copy_library()
    build_plugin()
    build_readme()


if __name__ == '__main__':
    try:
        main()
    except subprocess.CalledProcessError:
        exit(1)

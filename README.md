<h1 align="center">
    🪚<br>
    poetry-udeps
</h1>

<div align="center">
    <strong>Find unused dependencies in pyproject.toml.</strong>
</div>
<br>
<div align="center">
  <a href="https://github.com/lukehsiao/poetry-udeps/actions/workflows/general.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/lukehsiao/poetry-udeps/general.yml" alt="Build Status">
  </a>
  <a href="https://crates.io/crates/poetry-udeps">
    <img src="https://img.shields.io/crates/v/poetry-udeps" alt="Version">
  </a>
  <a href="https://github.com/lukehsiao/poetry-udeps/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/poetry-udeps" alt="License">
  </a>
</div>
<br>

`poetry-udeps` is inspired by [`cargo-udeps`](https://github.com/est31/cargo-udeps) and is a tool for finding unused dependencies in a [Poetry](https://python-poetry.org/)-based Python project.
That is, finding unused dependencies in `pyproject.toml`.

Python dependencies do not always map 1:1 with their package names.
Consequently, it is _likely_ that you will see false positives.
Hopefully, the list of positives is small enough for this tool to be useful, and to be easy to manually audit.

Additional name mappings can be added to [`src/name_map.rs`](src/name_map.rs) to improve accuracy.

**Contents**

-   [Install](#install)
    -   [From crates.io](#from-crates.io)
-   [Usage](#usage)
-   [How does this work?](#how-does-this-work)
-   [Related Tools](#related-tools)
    -   [Benchmarks](#benchmarks)
-   [Trophy Case](#trophy-case)
-   [License](#license)

## Install

This tool expects that you run it where you have `poetry` on your `$PATH`.

### From crates.io

```
cargo install poetry-udeps --locked
```

## Usage

This is meant to be run in the root of your Poetry project.

```
Find unused dependencies in pyproject.toml

Usage: poetry-udeps [OPTIONS]

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -e, --virtualenv  Whether to look for dependency usage in the poetry
                    virtualenv
  -d, --dev         Whether to look for unused dependencies from
                    dev-dependencies
  -h, --help        Print help (see more with '--help')
  -V, --version     Print version
```

## How does this work?

This is a very simple parsing approach.
That is, `poetry-udeps` doesn't interpret any Python, we just literally search through all the files in the project for import statements which match the package names (or their aliases as defined by the embedded name map).
This means it is fast!
But, it also means there are some false positives that it simply cannot detect.
For example, sqlalchemy's async sessions might depend on `asyncpg`, even though your immediate project never imports it.
To help with that (somewhat), you can use the option (`--virtualenv`) to include searching through all the Python files in your poetry environment as well.

## Related Tools

- [deptry](https://github.com/fpgmaas/deptry) (python/rust): Find unused, missing and transitive dependencies in a Python project.
- [pip-extra-reqs](https://github.com/r1chardj0n3s/pip-check-reqs) (python): find packages that should be in requirements for a project.
- [fawltydeps](https://github.com/tweag/FawltyDeps) (python): Python dependency checker.
- [py-unused-deps](https://github.com/matthewhughes934/py-unused-deps) (python): Find unused dependencies in your Python packages.
- [un-pack](https://github.com/bnkc/unpack) (rust): Unpack python packages from your project and more.

### Benchmarks

`poetry-udeps` only checks for unused dependencies.
Below, we benchmark this single feature on a desktop with an AMD Ryzen 7 7800X3D and 64 GB of RAM.
The target repository is a private repository consisting of ~170k lines of Python code.

```
❯ tokei -C -t Python
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Python                904       194995       167640         9686        17669
===============================================================================
 Total                 904       194995       167640         9686        17669
===============================================================================
```

#### Results

##### poetry-udeps
```
❯ hyperfine --warmup 2 -i 'poetry-udeps'
Benchmark 1: poetry-udeps
  Time (mean ± σ):     110.3 ms ±   0.7 ms    [User: 203.2 ms, System: 15.8 ms]
  Range (min … max):   108.9 ms … 111.6 ms    27 runs

  Warning: Ignoring non-zero exit code.
```

##### deptry
For `deptry`, only `DEP002` (unused dependencies) is considered.
Note this is running deptry v0.14.0, with core parts rewritten in Rust.

```
❯ hyperfine --warmup 2 -i 'deptry -i DEP001,DEP003,DEP004 .'
Benchmark 1: deptry -i DEP001,DEP003,DEP004 .
  Time (mean ± σ):     165.2 ms ±   1.8 ms    [User: 389.4 ms, System: 38.9 ms]
  Range (min … max):   161.6 ms … 168.8 ms    18 runs
```

##### pip-extra-reqs
`pip-extra-reqs` was unable to run on this project.

```
❯ pip-extra-reqs .
Traceback (most recent call last):
  File "/home/lukehsiao/repos/redacted/.venv/bin/pip-extra-reqs", line 8, in <module>
    sys.exit(main())
  File "/home/lukehsiao/repos/redacted/.venv/lib/python3.10/site-packages/pip_check_reqs/find_extra_reqs.py", line 234, in main
    extras = find_extra_reqs(
  File "/home/lukehsiao/repos/redacted/.venv/lib/python3.10/site-packages/pip_check_reqs/find_extra_reqs.py", line 62, in find_extra_reqs
    used_modules = common.find_imported_modules(
  File "/home/lukehsiao/repos/redacted/.venv/lib/python3.10/site-packages/pip_check_reqs/common.py", line 154, in find_imported_modules
    content = filename.read_text(encoding="utf-8")
  File "/home/lukehsiao/.pyenv/versions/3.10.13/lib/python3.10/pathlib.py", line 1135, in read_text
    return f.read()
  File "/home/lukehsiao/.pyenv/versions/3.10.13/lib/python3.10/codecs.py", line 322, in decode
    (result, consumed) = self._buffer_decode(data, self.errors, final)
UnicodeDecodeError: 'utf-8' codec can't decode byte 0xb1 in position 81: invalid start byte
```

##### fawltydeps
```
❯ hyperfine --warmup 2 -i 'fawltydeps --check-unused --deps pyproject.toml'
Benchmark 1: fawltydeps --check-unused --deps pyproject.toml
  Time (mean ± σ):      3.570 s ±  0.015 s    [User: 3.179 s, System: 0.379 s]
  Range (min … max):    3.549 s …  3.595 s    10 runs
```

##### py-unused-deps

I was unable to successfully run `py-unused-deps` on this project.

## Trophy Case

This is a list of cases where unused dependencies were found using `poetry-udeps`. You are welcome to expand it:

- TODO

## License

This tool is distributed under the terms of the Blue Oak license.
Any contributions are licensed under the same license, and acknowledge via the [Developer Certificate of Origin](https://developercertificate.org/).

See [LICENSE](LICENSE.md) for details.

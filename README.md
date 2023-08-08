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
    <img src="https://img.shields.io/github/actions/workflow/status/lukehsiao/poetry-udeps/general.yml" alt="Build Status"></a>
  <a href="https://crates.io/crates/poetry-udeps">
    <img src="https://img.shields.io/crates/v/poetry-udeps" alt="Version">
  </a>
  <img src="https://img.shields.io/crates/l/poetry-udeps" alt="License">
</div>
<br>

`poetry-udeps` is inspired by [`cargo-udeps`](https://github.com/est31/cargo-udeps) and is a tool for finding unused dependencies in a [Poetry](https://python-poetry.org/)-based Python project.
That is, finding unused dependencies in `pyproject.toml`.

Python dependencies do not always map 1:1 with their package names. 
Consequently, it is _likely_ that you will see false positives.
Hopefully, the list of positives is small enough for this tool to be useful, and to be easy to manually audit.

Additional name mappings can be added to [`src/name_map.rs`](src/name_map.rs) to improve accuracy.

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

## Related Tools

- [deptry](https://github.com/fpgmaas/deptry) (python): Find unused, missing and transitive dependencies in a Python project. 
- [pip-extra-reqs](https://github.com/r1chardj0n3s/pip-check-reqs) (python): find packages that should be in requirements for a project 

### Benchmarks

`poetry-udeps` only checks for unused dependencies.
Below, we benchmark this single feature on a desktop with an AMD Ryzen 7 3700X and 16 GB of RAM.
The target repository is a private repository consisting of ~100k lines of Python code.

```
❯ tokei -C -t Python
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Python                699       118239       100522         5464        12253
===============================================================================
 Total                 699       118239       100522         5464        12253
===============================================================================
```

#### Results

```
❯ hyperfine --warmup 2 'poetry-udeps'
Benchmark 1: poetry-udeps
  Time (mean ± σ):     116.2 ms ±   2.9 ms    [User: 217.9 ms, System: 12.9 ms]
  Range (min … max):   111.6 ms … 123.5 ms    26 runs
```

For `deptry`, only `DEP002` (unused dependencies) is considered.

```
❯ hyperfine --warmup 2 -i 'deptry -i DEP001,DEP003,DEP004 .'
Benchmark 1: deptry -i DEP001,DEP003,DEP004 .
  Time (mean ± σ):      1.065 s ±  0.020 s    [User: 1.038 s, System: 0.026 s]
  Range (min … max):    1.043 s …  1.116 s    10 runs
```

`pip-extra-reqs` was unable to run on this project.

```
❯ pip-extra-reqs .
Traceback (most recent call last):
  File "/home/benchmark/.venv/bin/pip-extra-reqs", line 8, in <module>
    sys.exit(main())
  File "/home/benchmark/.venv/lib/python3.10/site-packages/pip_check_reqs/find_extra_reqs.py", line 211, in main
    extras = find_extra_reqs(
  File "/home/benchmark/.venv/lib/python3.10/site-packages/pip_check_reqs/find_extra_reqs.py", line 35, in find_extra_reqs
    used_modules = common.find_imported_modules(
  File "/home/benchmark/.venv/lib/python3.10/site-packages/pip_check_reqs/common.py", line 153, in find_imported_modules
    vis.visit(ast.parse(content, str(filename)))
  File "/home/lukehsiao/.pyenv/versions/3.10.5/lib/python3.10/ast.py", line 50, in parse
    return compile(source, filename, mode, flags,
  File "/home/benchmark/.venv/lib/python3.10/site-packages/uuid.py", line 138
    if not 0 <= time_low < 1<<32L:
                               ^
SyntaxError: invalid decimal literal
```

## Trophy Case

This is a list of cases where unused dependencies were found using `poetry-udeps`. You are welcome to expand it:

- TODO

## License

This tool is distributed under the terms of the Blue Oak license.
Any contributions are licensed under the same license, and acknowledge via the [Developer Certificate of Origin](https://developercertificate.org/).

See [LICENSE](LICENSE) for details.

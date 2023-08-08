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

### Related Tools

- [deptry](https://github.com/fpgmaas/deptry) ![python](https://img.shields.io/badge/lang-python-cornflowerblue): Find unused, missing and transitive dependencies in a Python project. 
- [pip-check-reqs](https://github.com/r1chardj0n3s/pip-check-reqs) ![python](https://img.shields.io/badge/lang-python-cornflowerblue): find packages that should be in requirements for a project 

## Trophy Case

This is a list of cases where unused dependencies were found using `poetry-udeps`. You are welcome to expand it:

- TODO

## License

This tool is distributed under the terms of the Blue Oak license.
Any contributions are licensed under the same license, and acknowledge via the [Developer Certificate of Origin](https://developercertificate.org/).

See [LICENSE](LICENSE) for details.

## Roadmap

- Better logging. E.g., the ability to increase verbosity and see where dependencies are referenced
  as they are removed from the set.
- Ability to distinguish if deps were removed from the set by the projects deps (i.e., during the
  venv scan), or by the project itself.
- Ability to parse jupyter notebooks. This would likely introduce a dependency like jupyter to use
  `nbconvert`. Lower priority.

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


### Related Tools

- [deptry](https://github.com/fpgmaas/deptry) ![python](https://img.shields.io/badge/lang-python-cornflowerblue): Find unused, missing and transitive dependencies in a Python project. 
- [pip-check-reqs](https://github.com/r1chardj0n3s/pip-check-reqs) ![python](https://img.shields.io/badge/lang-python-cornflowerblue): find packages that should be in requirements for a project 

## Trophy Case

Currently empty.

## Roadmap

- Better logging. E.g., the ability to increase verbosity and see where dependencies are referenced
  as they are removed from the set.
- Ability to distinguish if deps were removed from the set by the projects deps (i.e., during the
  venv scan), or by the project itself.
- Ability to parse jupyter notebooks. This would likely introduce a dependency like jupyter to use
  `nbconvert`. Lower priority.

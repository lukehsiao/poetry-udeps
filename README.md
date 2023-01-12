<h1 align="center">
    🪚<br>
    poetry-udeps
</h1>

<div align="center">
    <strong>Find unused dependencies in pyproject.toml.</strong>
</div>
<br>
<br>

TODO

- nom parser combinator to parse the `from *` and `import *` names, while skipping appearances of
  those in comments.
- Logic for getting module names from packages
  (https://stackoverflow.com/questions/11453866/given-the-name-of-a-python-package-what-is-the-name-of-the-module-to-import/54853084#54853084)
    - An additional map for manual mappings, perhaps?
- Parsing pyproject.toml for dependencies
- Starting repo scans from either pyproject.toml dir, or the dirs specified in packages.
- Performance comparisons to deptry and pip-extra-reqs
- Better logging. E.g., the ability to increase verbosity and see where depenencies are referenced
  as they are removed from the set.
- Ability to distinguish if deps were removed from the set by the projects deps (i.e., during the
  venv scan), or by the project itself.
- Ability to parse jupyter notebooks. This would likely introduce a dependency like jupyter to use
  `nbconvert`. Lower priority.

# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.1.4](https://github.com/lukehsiao/poetry-udeps/compare/v0.1.3..v0.1.4) - 2024-01-18

### Documentation

- **(README)** link license badge to license - ([c6a4229](https://github.com/lukehsiao/poetry-udeps/commit/c6a4229d8feb1d3d2234547a3cc9a4a40144a3ab)) - Luke Hsiao

---
## [0.1.3](https://github.com/lukehsiao/poetry-udeps/compare/v0.1.2..vv0.1.3) - 2023-08-08

### Bug Fixes

- support poetry 1.2+'s dependency groups - ([97dc80d](https://github.com/lukehsiao/poetry-udeps/commit/97dc80d65f27721fe19f4973189b197af2539ea7)) - Luke Hsiao

### Documentation

- **(CHANGELOG)** add entry for v0.1.3 - ([4eb03cf](https://github.com/lukehsiao/poetry-udeps/commit/4eb03cf971ba06722e7beeb69f71dffd7823eddf)) - Luke Hsiao
- **(README)** set expectation of false positives - ([22defbf](https://github.com/lukehsiao/poetry-udeps/commit/22defbf823cc3b3b0933286262b90dd651806f4f)) - Luke Hsiao
- **(README)** add fawltydeps, py-unused-deps - ([910671d](https://github.com/lukehsiao/poetry-udeps/commit/910671d166cf5225aadd8a07d3db4936b73182bc)) - Luke Hsiao

---
## [0.1.2](https://github.com/lukehsiao/poetry-udeps/compare/v0.1.1..vv0.1.2) - 2023-08-08

### Documentation

- **(CHANGELOG)** add entry for v0.1.2 - ([9b7ee8a](https://github.com/lukehsiao/poetry-udeps/commit/9b7ee8a6c22fc2d15f40da1212405c7f2aa1c8ce)) - Luke Hsiao

### Refactor

- clean up info-level log formatting - ([33c67bc](https://github.com/lukehsiao/poetry-udeps/commit/33c67bc533e17cfedaac6653b5364cd684574b53)) - Luke Hsiao

---
## [0.1.1](https://github.com/lukehsiao/poetry-udeps/compare/v0.1.0..vv0.1.1) - 2023-08-08

### Bug Fixes

- replace `.`, not `,` for google-style naming - ([b0a0827](https://github.com/lukehsiao/poetry-udeps/commit/b0a08270da535fea6bf81a5f63cbf4784f0f9e41)) - Luke Hsiao
- don't include `python` in the dependencies - ([4c50a31](https://github.com/lukehsiao/poetry-udeps/commit/4c50a31deb6b8952829273385538767c0792f83d)) - Luke Hsiao

### Documentation

- **(CHANGELOG)** add initial changelog - ([3728af4](https://github.com/lukehsiao/poetry-udeps/commit/3728af4a9cb8ce2544c30571e6ae8c4c7f430028)) - Luke Hsiao
- **(CHANGELOG)** add entry for v0.1.1 - ([1aa86ef](https://github.com/lukehsiao/poetry-udeps/commit/1aa86ef5eaa4a1eb7cea18ca839399a12d76bad5)) - Luke Hsiao
- **(README)** add benchmark - ([4943c69](https://github.com/lukehsiao/poetry-udeps/commit/4943c698bb68eafccca1a9a5feaef1c54b55588b)) - Luke Hsiao

### Features

- **(name-map)** include more common packages - ([7e7cc6d](https://github.com/lukehsiao/poetry-udeps/commit/7e7cc6dee6a7a18456c9152a569f010d419ffe8f)) - Luke Hsiao
- add debug-level logs for seeing what is checked - ([b247e87](https://github.com/lukehsiao/poetry-udeps/commit/b247e87d18855d5dde727441092d43f1b1a75b20)) - Luke Hsiao
- check two-level package names for better Google support - ([9559064](https://github.com/lukehsiao/poetry-udeps/commit/95590641da9b5887a38b7c3d953d5ff58e03a751)) - Luke Hsiao

---
## [0.1.0] - 2023-08-08

### Bug Fixes

- ensure map entries are ordered - ([473edf3](https://github.com/lukehsiao/poetry-udeps/commit/473edf384b22c4e332149cc7a66096ba0d7356ae)) - Luke Hsiao

### Documentation

- **(README)** add some TODOs for future reference - ([667b472](https://github.com/lukehsiao/poetry-udeps/commit/667b4722d347caf298b26c5ce0c8b1508d5b568d)) - Luke Hsiao
- **(README)** add badge placeholders - ([ffcc7e5](https://github.com/lukehsiao/poetry-udeps/commit/ffcc7e5143a0a7ade5128330c81d66e202bde2df)) - Luke Hsiao
- **(README)** add sections - ([17da2ec](https://github.com/lukehsiao/poetry-udeps/commit/17da2ecf6c1a8a24ba784c0f0676132122c04f21)) - Luke Hsiao
- **(README)** populate more sections - ([160d67b](https://github.com/lukehsiao/poetry-udeps/commit/160d67b3d406414e884745f95ac11dc1c05be8a3)) - Luke Hsiao
- **(README)** drop language badges for plain text - ([aee0c5b](https://github.com/lukehsiao/poetry-udeps/commit/aee0c5b3e0b0a2d6956d728afd9381273c42974a)) - Luke Hsiao
- **(README)** add benchmarks section - ([082c5cb](https://github.com/lukehsiao/poetry-udeps/commit/082c5cb0b36568fd17f8243df0d0834b6e1922df)) - Luke Hsiao
- **(changelog)** change git-cliff template - ([70279f7](https://github.com/lukehsiao/poetry-udeps/commit/70279f79f8aea46f67500e9d44bf373cb5e33e80)) - Luke Hsiao

### Refactor

- add fast recursive directory traversal - ([190203f](https://github.com/lukehsiao/poetry-udeps/commit/190203fdc77ed7434bf94447707961d4ecea0895)) - Luke Hsiao
- drop scanning jupyter notebooks for now - ([3a80557](https://github.com/lukehsiao/poetry-udeps/commit/3a80557cc567eeb2e1a24ea75609e53767325f89)) - Luke Hsiao
- parse dependencies from pyproject.toml - ([4b80a72](https://github.com/lukehsiao/poetry-udeps/commit/4b80a72d43ae56fac1dfda28cf3af139dbb3cb88)) - Luke Hsiao
- use a static map for getting deps - ([36af56a](https://github.com/lukehsiao/poetry-udeps/commit/36af56aad99676b742d9b8a91b0327471f798aa6)) - Luke Hsiao
- update nom parsers - ([935a9cf](https://github.com/lukehsiao/poetry-udeps/commit/935a9cf0d4bf35065afe25648d3b69f3145c7a23)) - Luke Hsiao
- use tracing for structured logging - ([4d7120b](https://github.com/lukehsiao/poetry-udeps/commit/4d7120bd34543d6a50d1aa13b56d5002cdbf72b4)) - Luke Hsiao
- add one variant of boto3-stubs - ([3de0e22](https://github.com/lukehsiao/poetry-udeps/commit/3de0e226e28d80bc7b06912e1e625ffec1911e25)) - Luke Hsiao

### WIP

- adding nom parser combinator for getting packages - ([a8aa949](https://github.com/lukehsiao/poetry-udeps/commit/a8aa9490fbc5aeecbc565351877fe118465ac487)) - Luke Hsiao

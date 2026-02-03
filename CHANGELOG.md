# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

## [1.4.3](https://github.com/seyallius/treeclip.v2/compare/v1.4.2...v1.4.3) - 2026-02-03

### Fixed

- separate release-plz and build-binaries workflows

## [1.4.2](https://github.com/seyallius/treeclip.v2/compare/v1.4.1...v1.4.2) - 2026-02-03

### Fixed

- avoid build-binaries and create-release trigger on push

## [1.4.1](https://github.com/seyallius/treeclip.v2/compare/v1.4.0...v1.4.1) - 2026-02-03

### Fixed

- build binaries wait for `release-plz-release`
- build binaries workflow

## [1.4.0](https://github.com/seyallius/treeclip.v2/compare/v1.3.0...v1.4.0) - 2026-02-02

### Added

- add init command with smart ignore file management

### Fixed

- change --skip-hidden flag to opt-in, not opt-out
- allow building binaries on each push to main (testing)
- home readme installation update

### Other

- update documentation with init command

## [1.3.0](https://github.com/seyallius/treeclip.v2/compare/v1.2.5...v1.3.0) - 2026-01-21

### Added

- add cross-platform binary builds and installers

### Other

- add future plans item
- update documentation with plans and license

## [1.2.5](https://github.com/seyallius/treeclip.v2/compare/v1.2.4...v1.2.5) - 2026-01-04

### Other

- update README to direct users to published documentation
- add gitbook docs

## [1.2.4](https://github.com/seyallius/treeclip.v2/compare/v1.2.3...v1.2.4) - 2026-01-04

### Other

- fix all clippy warnings and errors
- enhance rayon integration for additional parallel processing
- improve console UI with better color consistency and layout

## [1.2.3](https://github.com/seyallius/treeclip.v2/compare/v1.2.2...v1.2.3) - 2026-01-02

### Fixed

- always read common ignore files
- fix tree traversal scope
- prevent gitignore duplication

## [1.2.2](https://github.com/seyallius/treeclip.v2/compare/v1.2.1...v1.2.2) - 2026-01-01

### Fixed

- merge multiple input paths into unified tree structure

## [1.2.1](https://github.com/seyallius/treeclip.v2/compare/v1.2.0...v1.2.1) - 2025-12-30

### Fixed

- add tree option flag in readme

## [1.2.0](https://github.com/seyallius/treeclip.v2/compare/v1.1.1...v1.2.0) - 2025-12-30

### Added

- add tree option (wip)

### Other

- tree message

## [1.1.1](https://github.com/seyallius/treeclip.v2/compare/v1.1.0...v1.1.1) - 2025-12-29

### Fixed

- output color

## [1.1.0](https://github.com/seyallius/treeclip.v2/compare/v1.0.0...v1.1.0) - 2025-12-29

### Added

- add automatic gitignore reading (wip)

### Fixed

- appending output file

## [0.10.0](https://github.com/seyallius/treeclip.v2/compare/v0.9.7...v0.10.0) - 2025-12-27

### Added

- [**breaking**] implement multiple input paths as option

### Other

- Update README.md

## [0.9.7](https://github.com/seyallius/treeclip.v2/compare/v0.9.6...v0.9.7) - 2025-12-26

### Fixed

- add glob note section

### Other

- add exclude pattern tests to make sure it works
- add cargo and rust cache to speed up release-plz workflow
- Add option to avoid overwriting output file
- Document multiple inputs and commands completion

## [0.9.6](https://github.com/seyallius/treeclip.v2/compare/v0.9.5...v0.9.6) - 2025-12-25

### Added

- comprehensive clap enhancements for professional UX

### Other

- add note in readme

## [0.9.5](https://github.com/seyallius/treeclip.v2/compare/v0.9.4...v0.9.5) - 2025-12-25

### Fixed

- rename repository username

## [0.9.4](https://github.com/seyedali-dev/treeclip.v2/compare/v0.9.3...v0.9.4) - 2025-12-24

### Fixed

- oops was using rc version which I didn't need it

## [0.9.3](https://github.com/seyedali-dev/treeclip.v2/compare/v0.9.2...v0.9.3) - 2025-12-24

### Other

- restructure usage patterns into comprehensive table format
- comprehensive usage guide with conversational tone
- add if block in release ci

## [0.9.2](https://github.com/seyedali-dev/treeclip.v2/compare/v0.9.1...v0.9.2) - 2025-12-24

### Other

- use pull requests for release process

## [0.9.1](https://github.com/seyedali-dev/treeclip.v2/compare/v0.9.0...v0.9.1) - 2025-12-24

### Added

- release-plz
- *(release)* set up automated release versioning
- *(changelog)* set up changelog

### Other

- so what? i use ai cli tools as well. the free ones



### 📝 Other Commits

- Yeah i've used ai so what? i learned production ready code so bleh! · *seyedali* · [`9b3d002`](https://github.com/seyedali-dev/treeclip.v2/commit/9b3d0028457eb095b88dcb4616af643839e78e92)

- So what? i use ai cli tools as well. the free ones · *seyedali* · [`b8d6771`](https://github.com/seyedali-dev/treeclip.v2/commit/b8d677144735aadf6971282f5502c40d52876d5d)

## [0.9.0] - 2025-12-20



### ✨ Features

- **errors:** Implement production-ready error handling with thiserror · *seyedali* · [`fe125c5`](https://github.com/seyedali-dev/treeclip.v2/commit/fe125c5345b6f55ad5d4e0c2b2449ec7a6eacac3)

<details>
<summary>Details</summary>
Introduced comprehensive error handling following Rust best practices:

- Added custom error types with thiserror for all modules
- Implemented TreeClipError hierarchy with 5 domain-specific error types
- Added rich context messages with anyhow's .with_context()
- Lazy-loaded context for zero overhead in success path
- Comprehensive error tests for all error variants
- Updated all modules: clipboard, editor, exclude, walker, utils
- Added FileSystemError, ClipboardError, TraversalError, EditorError, PatternError
- Improved error messages with full context chains
- Added clipboard size limit (100MB) with proper error
- Better path validation with detailed error messages

Pattern used by Cargo, ripgrep, and other production Rust apps.
Zero breaking changes - only improved error quality.
</details>

## [0.8.1] - 2025-12-20



### 🐛 Bug Fixes

- **test:** Fix test hardcoding emoji which will fail due to randomness · *seyedali* · [`454690c`](https://github.com/seyedali-dev/treeclip.v2/commit/454690c90d96beeba8816addeb31b571e3065cd5)

## [0.8.0] - 2025-12-20



### ✨ Features

- **refactor:** Comprehensive code restructure with clean architecture · *seyedali* · [`ce71bab`](https://github.com/seyedali-dev/treeclip.v2/commit/ce71bab0f42d7038c26508a07e9a18de3a3e0924)

<details>
<summary>Details</summary>
Refactored entire codebase following SOLID principles and clean code practices:

- Applied Single Responsibility Principle across all modules
- Implemented Builder Pattern for FormattedBox
- Added comprehensive package and function-level docstrings
- Separated concerns: walker, clipboard, editor, UI components
- Added fast-mode flag for instant execution without animations
- Wrote 50+ unit tests covering core functionality
- Marked optimization opportunities with TODO/NOTE comments
- Improved error handling with context throughout
- Organized code with clear comment separators
- Ensured Unicode-safe formatting with proper width calculations

No breaking changes. All existing functionality preserved.
New feature: --fast-mode flag for CI/CD pipelines.

Performance improvements and memory optimization marked for future work.
</details>

## [0.7.5] - 2025-12-19



### 🐛 Bug Fixes

- Fix clippy all warnings `cargo clippy -- -W clippy::all` · *seyedali* · [`00951f8`](https://github.com/seyedali-dev/treeclip.v2/commit/00951f8f4d760db2a7bffdf19297a675c8827ca0)



### 📝 Other Commits

- Add table creating util · *seyedali* · [`31fd83b`](https://github.com/seyedali-dev/treeclip.v2/commit/31fd83b8edf77874ede0cb5577afb4248fcedb63)

- Make the ui kind of better · *seyedali* · [`5fd899f`](https://github.com/seyedali-dev/treeclip.v2/commit/5fd899fa201b31127a538a4ba1f500dba588d6c2)

- Reformat code with rust fmt · *seyedali* · [`03c8587`](https://github.com/seyedali-dev/treeclip.v2/commit/03c85875afcbe5ffb3309294b0192b4eb36e49c9)

## [0.7.4] - 2025-12-19



### 📝 Other Commits

- Ui/ux? · *seyedali* · [`b3c8d7a`](https://github.com/seyedali-dev/treeclip.v2/commit/b3c8d7a947d267fe654d82dc14ad0d7b945298b3)

- Ui/ux again?! · *seyedali* · [`a83cc83`](https://github.com/seyedali-dev/treeclip.v2/commit/a83cc837abd72ba1eb8c037e13abf291593042ae)



### 🔧 Miscellaneous Tasks

- **flag:** Add short and long for output · *seyedali* · [`94a9269`](https://github.com/seyedali-dev/treeclip.v2/commit/94a926966ccc2c8d8d43ff0752995b4a401fb10a)

## [0.7.3] - 2025-12-18



### 📝 Other Commits

- Not much important · *seyedali* · [`ead0a13`](https://github.com/seyedali-dev/treeclip.v2/commit/ead0a13e99a0c3b915fbf4f3d1122b12cb97b52a)

- Stdout format · *seyedali* · [`f70d70a`](https://github.com/seyedali-dev/treeclip.v2/commit/f70d70af2ff918b4f4880486c3e044f6b26ee4d3)

## [0.7.2] - 2025-12-17



### 📝 Other Commits

- Implement deleting output file flag · *seyedali* · [`e10bf9f`](https://github.com/seyedali-dev/treeclip.v2/commit/e10bf9f96c5089e8efdcd4a159921ca4f7be2640)

## [0.7.1] - 2025-12-16



### 📝 Other Commits

- Use gui editor instead of cli default! · *seyedali* · [`387e491`](https://github.com/seyedali-dev/treeclip.v2/commit/387e4911d758b35d29981ee73ea5273997f6138a)

## [0.7.0] - 2025-12-16



### 📝 Other Commits

- Implement editor flag · *seyedali* · [`7d3b209`](https://github.com/seyedali-dev/treeclip.v2/commit/7d3b2091c35fc4876e921a1a55f3f7acc71f99f8)

## [0.6.0] - 2025-12-16



### 📝 Other Commits

- Implement stats flag · *seyedali* · [`c108013`](https://github.com/seyedali-dev/treeclip.v2/commit/c108013d4981e19d5e407dc09cf40f0953d29ea6)

## [0.5.1] - 2025-12-13



### 🐛 Bug Fixes

- Fix arboard clipboard blocking the thread and not exiting · *seyedali* · [`b80fa30`](https://github.com/seyedali-dev/treeclip.v2/commit/b80fa30bd22b80b31de89ca55867116cc2b95a8d)

## [0.5.0] - 2025-12-13



### ✨ Features

- **clipboard:** Implement clipboard but it's not working quite well · *seyedali* · [`1c8d0f6`](https://github.com/seyedali-dev/treeclip.v2/commit/1c8d0f6a9fa0120797fd54aa69c09e21cf2d43a3)

<details>
<summary>Details</summary>
I have to copy something else to end the program. It is a limitation of arboard I think?
</details>

## [0.4.0] - 2025-12-12



### 📝 Other Commits

- Nimp · *seyedali* · [`2db442b`](https://github.com/seyedali-dev/treeclip.v2/commit/2db442b37b8b21e55dbb6c9ebf2c50b768ee030d)

- Implement file/folder exclusion while traversal · *seyedali* · [`db7f39f`](https://github.com/seyedali-dev/treeclip.v2/commit/db7f39f1f76dbffa7d80e6a9de26626f0bbba680)

## [0.3.3] - 2025-12-12



### 📝 Other Commits

- Some refactoring shit · *seyedali* · [`d53f8ce`](https://github.com/seyedali-dev/treeclip.v2/commit/d53f8cee90ab60373af0bd54c28853db9964d3fb)

- Make the readme correct with help of qwen cuz i have a lazy ass · *seyedali* · [`8a1ba0c`](https://github.com/seyedali-dev/treeclip.v2/commit/8a1ba0cde711956956ea84d23b6780048cc5c5aa)

- Avoid reading output file just to add trim end and add \n · *seyedali* · [`9b08570`](https://github.com/seyedali-dev/treeclip.v2/commit/9b08570c93e9bf82c7cb598a172080b9b9275f2e)

## [0.3.2] - 2025-12-11



### 📝 Other Commits

- It's now traversing and writing in the desired output ^O^ · *seyedali* · [`0260fb6`](https://github.com/seyedali-dev/treeclip.v2/commit/0260fb6add4838e3dc14d616f93a2fc13aeaed70)

## [0.3.1] - 2025-12-10



### 📝 Other Commits

- **write:** Write extracted content · *seyedali* · [`395d884`](https://github.com/seyedali-dev/treeclip.v2/commit/395d8847459a9e4c912858c3b3960af29eb3c0b2)

## [0.3.0] - 2025-12-10



### ♻️ Refactoring

- **structure:** Yup, yet another refactor ahaha :) · *seyedali* · [`4bedad9`](https://github.com/seyedali-dev/treeclip.v2/commit/4bedad9b3ae010c848bdd94f8451b032bd076f19)



### ✨ Features

- **skip_hidden:** Add skip hidden items flag · *seyedali* · [`0ac257b`](https://github.com/seyedali-dev/treeclip.v2/commit/0ac257b5c23ff80ac100eabc071d5ef6822608a0)

- **raw-cmd:** Add raw cmd for later · *seyedali* · [`7f1da13`](https://github.com/seyedali-dev/treeclip.v2/commit/7f1da13bc1bdb84af931de87584eeda6019b48bd)



### 🔧 Miscellaneous Tasks

- **gitignore:** Ignore previous treeclip · *seyedali* · [`9143e9d`](https://github.com/seyedali-dev/treeclip.v2/commit/9143e9d21b1894f1e1b5c7d1ec4e61bda17ff1f3)



### 🧪 Testing

- **unit-test:** Add unit tests · *seyedali* · [`1b6a2d5`](https://github.com/seyedali-dev/treeclip.v2/commit/1b6a2d55e8f72dc94c9d38377b0d2c31688d41b2)

## [0.2.1] - 2025-12-05



### ♻️ Refactoring

- **command:** Refactor commands · *seyedali* · [`e7fc9fc`](https://github.com/seyedali-dev/treeclip.v2/commit/e7fc9fc87b3c60f708a7f267498c4f1fb0dab4fe)

- **command:** Command separation · *seyedali* · [`481e19d`](https://github.com/seyedali-dev/treeclip.v2/commit/481e19d077b07e8a09fee5ce0f49814504950db3)



### 🐛 Bug Fixes

- **input:** Wrong usage of input path in run function · *seyedali* · [`9f51945`](https://github.com/seyedali-dev/treeclip.v2/commit/9f51945e70b428d2b3c4b27a9907262bfc30bbd8)



### 🔧 Miscellaneous Tasks

- **imports:** Refactor module imports · *seyedali* · [`8bb6a8f`](https://github.com/seyedali-dev/treeclip.v2/commit/8bb6a8f7d9ccc60bd52be0c233937efed704c0e9)

## [0.2.0] - 2025-12-02



### ✨ Features

- **path-cmd:** Basic path command printing specified path's contents · *seyedali* · [`45e308d`](https://github.com/seyedali-dev/treeclip.v2/commit/45e308d76deeb52ae53a23b21eba11353262983b)

## [0.1.0] - 2025-12-02



### ✨ Features

- **setup:** Cargo.toml base setup · *seyedali* · [`88e18b5`](https://github.com/seyedali-dev/treeclip.v2/commit/88e18b5333807e406f106bcfd56c87983e8efe92)



### 📝 Other Commits

- Initial commit · *SeyedAli* · [`2335d40`](https://github.com/seyedali-dev/treeclip.v2/commit/2335d403f95ea06a95800560c40a724950f4cd47)



### 🔧 Miscellaneous Tasks

- **.idea:** Ignore .idea · *seyedali* · [`82dacca`](https://github.com/seyedali-dev/treeclip.v2/commit/82dacca503746e60af200e246eb05ab1199f8acf)

<!-- generated by git-cliff -->

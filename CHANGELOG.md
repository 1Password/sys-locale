# sys-locale changelog

Notable changes to this project will be documented in the [keep a changelog](https://keepachangelog.com/en/1.0.0/) format.

## [Unreleased]

## [0.2.2] - 2022-11-06

### Changed
- The Apple backend has been rewritten in pure Rust instead of Objective-C.

### Fixed
- The locale returned on UNIX systems is now always a correctly formatted BCP-47 tag.

## [0.2.1] - 2022-06-16

### New

- The crate now supports being used via WASM in a WebWorker environment.

## [0.2.0] - 2022-03-01

### Fixed

- Fixed a soundness issue on Linux and BSDs by querying the environment directly instead of using libc setlocale. The libc setlocale is not safe for use in a multi-threaded context.

### Changed

- No longer `no_std` on Linux and BSDs

## [0.1.0] - 2021-05-13

Initial release

[Unreleased]: https://github.com/1Password/sys-locale/compare/v0.2.2...HEAD
[0.1.0]: https://github.com/1Password/sys-locale/releases/tag/v0.1.0
[0.2.0]: https://github.com/1Password/sys-locale/releases/tag/v0.2.0
[0.2.1]: https://github.com/1Password/sys-locale/releases/tag/v0.2.1
[0.2.2]: https://github.com/1Password/sys-locale/releases/tag/v0.2.2

# sys-locale changelog

Notable changes to this project will be documented in the [keep a changelog](https://keepachangelog.com/en/1.0.0/) format.

## [Unreleased]

## [0.2.0] - 2022-02-28

### Fixed

- Fixed a soundness issue on Linux and BSDs by querying the environment directly instead of using libc setlocale. The libc setlocale is not safe for use in a multi-threaded context.

### Changed

- No longer `no_std` on Linux and BSDs

## [0.1.0] - 2021-05-13

Initial release

[Unreleased]: https://github.com/1Password/sys-locale/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/1Password/sys-locale/releases/tag/v0.1.0
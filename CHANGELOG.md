# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### Added
- Lists of Founding Modder and Developer Steam IDs.
- Returned iterators now implement `Send`.

### Changed
- `format_score()`'s output more closely matches in-game time formatting, as it now omits hours in the output for times
under an hour.
- `create_leaderboard_name_string()` now returns a `Result` instead of an `Option`.

### Removed
- `Default` impl for `NegativeScoreError`.

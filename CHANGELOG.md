# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.3.1] - 2024-06-27

### Added

- `format_score_legacy()` function, which behaves like the `format_score()` from v0.1 of this library. It always includes hours, even for times under an hour.

## [0.3.0] - 2024-06-27

### Changed

- The official level lists have been updated with the new levels added in Distance v1.5.

## [0.2.0] - 2020-10-03

### Added

- Lists of Founding Modder and Developer Steam IDs.
- Returned iterators now implement `Send`.

### Changed

- `format_score()`'s output more closely matches in-game time formatting, as it now omits hours in the output for times under an hour.
- `create_leaderboard_name_string()` now returns a `Result` instead of an `Option`.

### Removed

- `Default` impl for `NegativeScoreError`.

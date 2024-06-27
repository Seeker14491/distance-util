//! Utility functionality useful when writing Rust tools for [Distance](http://survivethedistance.com/).
//!
//! The current functionality includes listing official levels, creating a level's leaderboard name
//! string, and formatting a raw time or score obtained from the Steamworks API.

#![warn(
    rust_2018_idioms,
    deprecated_in_future,
    macro_use_extern_crate,
    missing_debug_implementations,
    unused_qualifications
)]

mod format_score;
mod official_level_names;
mod special_steam_ids;

pub use format_score::*;
pub use special_steam_ids::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{
    error::Error,
    fmt::{self, Display},
};

/// All Distance game modes that include leaderboards.
///
/// The numeric value of each variant matches that game mode's id in the Distance game code.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LeaderboardGameMode {
    Sprint = 1,
    Stunt = 2,
    Challenge = 8,
}

impl LeaderboardGameMode {
    /// Returns the string representation of the game mode name.
    ///
    /// # Example
    ///
    /// ```
    /// use distance_util::LeaderboardGameMode;
    ///
    /// let s = LeaderboardGameMode::Sprint.name();
    /// assert_eq!(s, "Sprint");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            LeaderboardGameMode::Sprint => "Sprint",
            LeaderboardGameMode::Stunt => "Stunt",
            LeaderboardGameMode::Challenge => "Challenge",
        }
    }

    /// Equivalent to calling [`official_level_names()`] with `self`.
    pub fn official_level_names(self) -> &'static [&'static str] {
        official_level_names(self)
    }

    /// Equivalent to calling [`official_level_leaderboard_names()`] with `self`.
    pub fn official_level_leaderboard_names(self) -> impl Iterator<Item = String> + Send {
        official_level_leaderboard_names(self)
    }
}

impl Display for LeaderboardGameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.name())
    }
}

/// Returns the level names of all official levels for the given game mode.
///
/// # Example
///
/// ```
/// use distance_util::LeaderboardGameMode;
///
/// let names = distance_util::official_level_names(LeaderboardGameMode::Sprint);
/// assert_eq!(names[0], "Broken Symmetry");
/// ```
pub fn official_level_names(mode: LeaderboardGameMode) -> &'static [&'static str] {
    match mode {
        LeaderboardGameMode::Sprint => official_level_names::SPRINT,
        LeaderboardGameMode::Stunt => official_level_names::STUNT,
        LeaderboardGameMode::Challenge => official_level_names::CHALLENGE,
    }
}

/// Returns an iterator that yields the leaderboard name of all official levels for the given game
/// mode.
///
/// The returned name is used in the Steamworks API as a key to fetch a level's leaderboard.
///
/// # Example
///
/// ```
/// use distance_util::LeaderboardGameMode;
///
/// let mut names = distance_util::official_level_leaderboard_names(LeaderboardGameMode::Sprint);
/// assert_eq!(names.next().unwrap(), "Broken Symmetry_1_stable");
/// ```
pub fn official_level_leaderboard_names(
    mode: LeaderboardGameMode,
) -> impl Iterator<Item = String> + Send {
    official_level_names(mode)
        .iter()
        .map(move |name| create_leaderboard_name_string(name, mode, None).unwrap())
}

/// Creates a level's leaderboard name string, which is needed to get a level's leaderboard
/// from the Steamworks API.
///
/// For official levels, `level` is the level's name. For workshop levels, `level` is the level's
/// filename without the `.bytes` extension (which can be different from the level's title).
/// `author_steam_id` must be given for workshop levels, and `None` for official levels.
///
/// # Errors
///
/// The Steamworks API imposes a limit on the length of leaderboard names, so this function errors
/// if a sufficiently long level name is given.
pub fn create_leaderboard_name_string(
    level: &str,
    game_mode: LeaderboardGameMode,
    author_steam_id: Option<u64>,
) -> Result<String, LevelNameTooLongError> {
    let leaderboard_name = if let Some(id) = author_steam_id {
        format!("{}_{}_{}_stable", level, game_mode as u8, id)
    } else {
        format!("{}_{}_stable", level, game_mode as u8)
    };

    if leaderboard_name.len() <= 128 {
        Ok(leaderboard_name)
    } else {
        Err(LevelNameTooLongError { leaderboard_name })
    }
}

/// The error returned by [`create_leaderboard_name_string`] when the resulting leaderboard name
/// is too long.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LevelNameTooLongError {
    leaderboard_name: String,
}

impl Display for LevelNameTooLongError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The generated leaderboard name \"{}\" is invalid because it's too long. The maximum valid length is 128 bytes, but it has a length of {} bytes.",
            &self.leaderboard_name,
            self.leaderboard_name.len(),
        )
    }
}

impl Error for LevelNameTooLongError {}

impl LevelNameTooLongError {
    /// Consumes this error, returning the invalid leaderboard name.
    pub fn into_leaderboard_name(self) -> String {
        self.leaderboard_name
    }
}

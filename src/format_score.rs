use crate::LeaderboardGameMode;
use std::{
    error::Error,
    fmt::{self, Display},
};
use thousands::Separable;

/// The error returned when a negative score is passed in to [`format_score`].
#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct NegativeScoreError {
    score: i32,
}

impl Display for NegativeScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "The score '{}' is invalid because it's negative",
            self.score
        )
    }
}

impl Error for NegativeScoreError {}

/// Returns a string representation of a raw score obtained from the Steamworks API.
///
/// Returns an error if `score` is negative.
///
/// # Examples
///
/// ```
/// use distance_util::LeaderboardGameMode;
///
/// let sprint_time = distance_util::format_score(17767890, LeaderboardGameMode::Sprint).unwrap();
/// assert_eq!(sprint_time, "04:56:07.89");
///
/// let stunt_time = distance_util::format_score(123_456, LeaderboardGameMode::Stunt).unwrap();
/// assert_eq!(stunt_time, "123,456 eV");
/// ```
pub fn format_score(
    score: i32,
    game_mode: LeaderboardGameMode,
) -> Result<String, NegativeScoreError> {
    if score < 0 {
        return Err(NegativeScoreError { score });
    }

    let formatted = match game_mode {
        LeaderboardGameMode::Sprint | LeaderboardGameMode::Challenge => format_score_as_time(score),
        LeaderboardGameMode::Stunt => format!("{} eV", score.separate_with_commas()),
    };

    Ok(formatted)
}

fn format_score_as_time(score: i32) -> String {
    assert!(score >= 0);

    // `score` is in milliseconds
    let (hours, rem) = div_rem(score, 1000 * 60 * 60);
    let (minutes, rem) = div_rem(rem, 1000 * 60);
    let (seconds, rem) = div_rem(rem, 1000);
    let centiseconds = rem / 10;

    format!(
        "{:02}:{:02}:{:02}.{:02}",
        hours, minutes, seconds, centiseconds
    )
}

// Simultaneous truncated integer division and modulus.
#[inline]
fn div_rem(x: i32, other: i32) -> (i32, i32) {
    (x / other, x % other)
}

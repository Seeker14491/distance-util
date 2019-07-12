pub extern crate enumflags2;

use enumflags2::BitFlags;
use enumflags2_derive::EnumFlags;
use num_integer::Integer;
use thousands::Separable;

mod official_level_names;

#[derive(EnumFlags, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum LeaderboardGameMode {
    Sprint = 1,
    Stunt = 2,
    Challenge = 8,
}

impl LeaderboardGameMode {
    pub fn official_levels(self) -> &'static [&'static str] {
        match self {
            LeaderboardGameMode::Sprint => official_level_names::SPRINT,
            LeaderboardGameMode::Stunt => official_level_names::STUNT,
            LeaderboardGameMode::Challenge => official_level_names::CHALLENGE,
        }
    }

    // TODO: precompute
    pub fn official_level_leaderboard_names(self) -> impl Iterator<Item = String> {
        self.official_levels()
            .iter()
            .map(move |x| create_leaderboard_name_string(*x, self, None).unwrap())
    }
}

pub fn official_level_leaderboard_names(
    game_modes: impl Into<BitFlags<LeaderboardGameMode>>,
) -> Vec<String> {
    let mut v = Vec::new();
    for mode in game_modes.into().iter() {
        v.extend(mode.official_level_leaderboard_names());
    }

    v
}

/// Creates a level's leaderboard name string, which is needed to get a level's leaderboard
/// from the Steamworks API.
///
/// For official levels, `level` is the level's name. For workshop levels, `level` is the level's
/// filename without the `.bytes` extension (which can be different from the actual level name).
/// `steam_id_owner` must be given for workshop levels, and `None` for official levels.
///
/// Levels with very long filenames do not have a valid leaderboard name string, and so this
/// function returns `None` in that case.
pub fn create_leaderboard_name_string(
    level: &str,
    game_mode: LeaderboardGameMode,
    steam_id_owner: Option<u64>,
) -> Option<String> {
    let s = if let Some(id) = steam_id_owner {
        format!("{}_{}_{}_stable", level, game_mode as u8, id)
    } else {
        format!("{}_{}_stable", level, game_mode as u8)
    };

    if s.len() <= 128 {
        Some(s)
    } else {
        None
    }
}

/// Retuns a string representation of a raw score obtained from the Steamworks API.
pub fn format_score(score: i32, game_mode: LeaderboardGameMode) -> String {
    match game_mode {
        LeaderboardGameMode::Sprint | LeaderboardGameMode::Challenge => format_score_as_time(score),
        LeaderboardGameMode::Stunt => format!("{} eV", score.separate_with_commas()),
    }
}

fn format_score_as_time(score: i32) -> String {
    assert!(score >= 0);

    // `score` is in milliseconds
    let (hours, rem) = score.div_rem(&(1000 * 60 * 60));
    let (minutes, rem) = rem.div_rem(&(1000 * 60));
    let (seconds, rem) = rem.div_rem(&(1000));
    let centiseconds = rem / 10;

    format!(
        "{:02}:{:02}:{:02}.{:02}",
        hours, minutes, seconds, centiseconds
    )
}

#[cfg(test)]
mod test {
    use super::format_score_as_time;

    #[test]
    fn test_format_score_as_time() {
        assert_eq!(format_score_as_time(17767890), "04:56:07.89");
    }
}

//! Shorthands for common dice rolls.

use rand::Rng as _;

fn int_range(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..=max)
}

/// Generates a random number between 1 and 2 (inclusive).
pub fn d2() -> i32 {
    int_range(1, 2)
}

/// Generates a random number between 1 and 4 (inclusive).
pub fn d4() -> i32 {
    int_range(1, 4)
}

/// Generates a random number between 1 and 6 (inclusive).
pub fn d6() -> i32 {
    int_range(1, 6)
}

/// Generates a random number between 1 and 8 (inclusive).
pub fn d8() -> i32 {
    int_range(1, 8)
}

/// Generates a random number between 1 and 10 (inclusive).
pub fn d10() -> i32 {
    int_range(1, 10)
}

/// Generates a random number between 1 and 12 (inclusive).
pub fn d12() -> i32 {
    int_range(1, 12)
}

/// Generates a random number between 1 and 20 (inclusive).
pub fn d20() -> i32 {
    int_range(1, 20)
}

/// Generates a random number between 1 and 100 (inclusive).
pub fn d100() -> i32 {
    int_range(1, 100)
}

/// Returns the sum of `count` d6 dice.
pub fn d6_sum(count: usize) -> i32 {
    std::iter::repeat_with(d6).take(count).sum::<i32>()
}

/// The result of a "challenge roll".
/// See `challenge_roll`'s documentation.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ChallengeRollResult {
    /// The player's d6 roll plus the modifier was higher than both challenge d10 rolls.
    StrongHit,
    /// The player's d6 roll plus the modifier was higher than exactly one challenge d10 roll.
    WeakHit,
    /// The player's d6 roll plus the modifier was lower or equal to both challenge d10 rolls.
    Miss,
}

/// Does a challenge roll. This consists in rolling a d6 and adding a modifier value to it (usually the player's skill) and comparing it to two "challenge d10".
///
/// To win against a challenge d10, the d6 plus modifier must be higher than the d10's value.
/// There are three possible outcomes: Winning both (a `StrongHit`), winning only one (a `WeakHit`) and losing both (a `Miss`).
///
/// # Probabilities:
/// The probabilities of winning given a modifier are as follow:
/// | Modifier | Miss Chance | Weak Hit Chance | Strong Hit Chance |
/// |----------|-------------|-----------------|-------------------|
/// | -5       | 100.00%     | 0.00%           | 0.00%             |
/// | -4       | 96.80%      | 3.04%           | 0.16%             |
/// | -3       | 90.75%      | 8.44%           | 0.82%             |
/// | -2       | 82.48%      | 15.23%          | 2.29%             |
/// | -1       | 71.99%      | 23.01%          | 5.00%             |
/// | +0       | 59.18%      | 31.64%          | 9.18%             |
/// | +1       | 45.10%      | 39.56%          | 15.34%            |
/// | +2       | 33.13%      | 43.51%          | 23.36%            |
/// | +3       | 23.34%      | 43.38%          | 33.28%            |
/// | +4       | 15.15%      | 39.82%          | 45.03%            |
/// | +5       | 9.19%       | 31.37%          | 59.45%            |
/// | +6       | 5.05%       | 23.41%          | 71.54%            |
/// | +7       | 2.25%       | 15.19%          | 82.56%            |
/// | +8       | 0.85%       | 8.28%           | 90.87%            |
/// | +9       | 0.16%       | 2.97%           | 96.87%            |
/// | +10      | 0.00%       | 0.00%           | 100.00%           |
pub fn challenge_roll(modifier: i32) -> ChallengeRollResult {
    let player_roll = d6() + modifier;
    let challenge1 = d10();
    let challenge2 = d10();

    let won1 = player_roll > challenge1;
    let won2 = player_roll > challenge2;

    match (won1, won2) {
        (false, false) => ChallengeRollResult::Miss,
        (false, true) | (true, false) => ChallengeRollResult::WeakHit,
        (true, true) => ChallengeRollResult::StrongHit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_range() {
        for _ in 0..100 {
            let val = int_range(1, 3);
            assert!((1..=3).contains(&val));
        }
    }

    #[test]
    fn challenge_roll_range() {
        for _ in 0..100 {
            let miss = challenge_roll(-5);
            let strong = challenge_roll(10);
            assert_eq!(miss, ChallengeRollResult::Miss);
            assert_eq!(strong, ChallengeRollResult::StrongHit);
        }
    }
}

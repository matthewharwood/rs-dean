use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Rating {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CardPhase {
    #[default]
    New,
    Learning,
    Review,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CardState {
    pub phase: CardPhase,
    pub interval_minutes: u32,
    pub reps: u32,
}

#[must_use]
pub fn review(state: CardState, rating: Rating) -> CardState {
    let reps = state.reps.strict_add(1);
    match (state.phase, rating) {
        (_, Rating::Again) => CardState {
            phase: CardPhase::Learning,
            interval_minutes: 1,
            reps,
        },
        (CardPhase::New | CardPhase::Learning, Rating::Easy) => CardState {
            phase: CardPhase::Review,
            interval_minutes: 4 * 1_440,
            reps,
        },
        (CardPhase::New | CardPhase::Learning, _) => CardState {
            phase: CardPhase::Learning,
            interval_minutes: 10,
            reps,
        },
        (CardPhase::Review, Rating::Hard) => CardState {
            phase: CardPhase::Review,
            interval_minutes: state.interval_minutes.strict_add(1_440),
            reps,
        },
        (CardPhase::Review, Rating::Good) => CardState {
            phase: CardPhase::Review,
            interval_minutes: state.interval_minutes.strict_mul(2),
            reps,
        },
        (CardPhase::Review, Rating::Easy) => CardState {
            phase: CardPhase::Review,
            interval_minutes: state.interval_minutes.strict_mul(3),
            reps,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{CardPhase, CardState, Rating, review};

    #[test]
    fn easy_new_card_graduates() {
        let next = review(CardState::default(), Rating::Easy);
        assert_eq!(next.phase, CardPhase::Review);
        assert_eq!(next.reps, 1);
    }
}

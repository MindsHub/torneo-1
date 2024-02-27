use rand::{rngs::OsRng, Rng};
use template_torneo::LogTradimento;

const ITEMS: usize = 5;

// both accused: 5 sec -> weight: 0.65
// was accused:  7 sec -> weight: 1
// only accused: 0 sec -> weight: 0
// none accused: 1 sec -> weight: 0.5


pub fn test_1(_me: &LogTradimento, _other: &LogTradimento) -> bool {
    const BOTH_ACCUSED_WEIGHT: f64 = 0.7;
    const WAS_ACCUSED_WEIGHT:  f64 = 2.0;
    const ONLY_ACCUSED_WEIGHT: f64 = 0.3;
    const NONE_ACCUSED_WEIGHT: f64 = 1.3;

    let self_action_list:  Vec<bool> = _me.azioni_passate().iter().rev().take(ITEMS).copied().collect();
    let other_action_list: Vec<bool> = _other.azioni_passate().iter().rev().take(ITEMS).copied().collect();

    let mut success_count: f64 = 0.0;

    for i in 0..other_action_list.len() {
        let other_accused: bool = other_action_list[i];
        let self_accused:  bool = self_action_list[i];

        if other_accused && self_accused {          // both accused each other
            success_count += BOTH_ACCUSED_WEIGHT;
        } else if other_accused && !self_accused {  // other accused self
            success_count += WAS_ACCUSED_WEIGHT;
        } else if !other_accused && self_accused {  // other didn't accuse but self did
            success_count += ONLY_ACCUSED_WEIGHT;
        } else if !other_accused && !self_accused { // both didn't accuse
            success_count += NONE_ACCUSED_WEIGHT
        }
    }

    if other_action_list.len() != 0 {
        let p = success_count / other_action_list.len() as f64;

        if p > 1.0 {
            return true;
        } else {
            return OsRng.gen_bool(p);
        }
    } else {
        return true;
    }
}

pub fn test_2(_me: &LogTradimento, _other: &LogTradimento) -> bool {
    let other_action_list: Vec<bool> = _other.azioni_passate().iter().rev().take(ITEMS).copied().collect();

    for i in 0..other_action_list.len() {
        if other_action_list[i] {
            return true;
        }
    }

    return false;
}

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(_me: &LogTradimento, _other: &LogTradimento) -> bool {
    return test_2(_me, _other);
}

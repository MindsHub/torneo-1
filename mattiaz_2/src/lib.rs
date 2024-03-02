use rand::{rngs::OsRng, Rng};
use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(me: &LogTradimento, other: &LogTradimento) -> bool {
    //let v: Vec<&bool> = me.azioni_passate().iter().rev().take(2).collect();
    if other.numero_veri() / 75 >= other.numero_falsi() / 100 {
        true
    } else {
        OsRng.gen_bool(0.5)
    }
}

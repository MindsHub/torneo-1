use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(me: &LogTradimento, other: &LogTradimento) -> bool {
    if me.azioni_passate().is_empty() {
        return false;
    }

    if let Some(false) = other.azioni_passate().last() {
        return true;
    }
    return false;
}
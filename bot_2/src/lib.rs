use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(me: &LogTradimento, _other: &LogTradimento) -> bool {
    if let Some(true) = me.azioni_passate().last() {
        false
    }
    else{ false }
}

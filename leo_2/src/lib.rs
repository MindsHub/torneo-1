use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(_me: &LogTradimento, _other: &LogTradimento) -> bool {
    let x = 2;
    if x == 2 {
        return true;
    } else {
        return false;
    }
}

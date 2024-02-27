use rand::rngs::ThreadRng;
use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(_me: &LogTradimento, other: &LogTradimento, _rng: &mut ThreadRng) -> bool {
    if other.numero_veri() > other.numero_totali()/2{
        return true;
    }
    else{return false;}
}

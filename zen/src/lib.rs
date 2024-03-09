use rand::rngs::ThreadRng;
use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(me: &LogTradimento, other: &LogTradimento, rng: &mut ThreadRng) -> bool {
    if other.numero_totali() < 10 {
        //determinazione risposte avversario
        return false;
    } else if other.numero_totali() < 20 {
        return true;
    }
    if other.azioni_passate()[0..10] == other.azioni_passate()[10..20] {
        //algoritmo pattern pari o univoco
        if other.azioni_passate()[0..10].iter().all(|elemento| *elemento == false){
            return true;
        }
        if other.azioni_passate()[0..10].iter().all(|elemento| *elemento == true){
            return true;
        }
        if other.azioni_passate()[0..10].chunks(2).map(|elemento| elemento[1]).all(|elemento| elemento == false){
            return true;
        }
        if other.azioni_passate()[0..10].chunks(2).map(|elemento| elemento[1]).all(|elemento| elemento == true){
            return true;
        }else{
            return true;
        }
    }else{
        if let Some(false) = other.azioni_passate().last() {
            return false;
        }else{
            return true;
        }
    }
    
}

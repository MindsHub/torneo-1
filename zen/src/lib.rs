use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(me: &LogTradimento, other: &LogTradimento) -> bool {
    if other.numero_totali() < 10 {
        //determinazione risposte avversario
        return true;
    } else if other.numero_totali() < 20 {
        return false;
    }
    if other.azioni_passate()[0..10] == other.azioni_passate()[10..20] {
        //algoritmo pattern pari o univoco
        for i in 0..10 {
            if let Some(true) = other.azioni_passate().get(i) {
                //se è univoca in true ritorna true
                return false;
            }
        }
        if other.azioni_passate()[0..10]
            .iter()
            .all(|elemento| *elemento == false)
        {
            return false;
        }
        if other.azioni_passate()[0..10]
            .chunks(2)
            .map(|elemento| elemento[1])
            .all(|elemento| elemento == false)
        {
            return false;
        }
        if other.azioni_passate()[0..10]
            .chunks(2)
            .map(|elemento| elemento[1])
            .all(|elemento| elemento == true)
        {
            return false;
        }
    }
    return false;
}

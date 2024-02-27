use template_torneo::LogTradimento;

/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(_me: &LogTradimento, other: &LogTradimento) -> bool {
    if other.azioni_passate().len()<100{
        return false;
    }
    let v: Vec<bool> = other.azioni_passate().iter().rev().take(100).copied().collect();
    let i  = v.iter().filter(|x| **x).count();
    //println!("{}", i);
    if i>5{
        true
    }else{
        false
    }
    //OsRng.gen_bool(i as f64/20.0)
}

use rand::{rngs::ThreadRng, Rng};
use template_torneo::LogTradimento;

static mut RNG: Option<ThreadRng> = None;//ThreadRng::default();
/// Scegliete quando incolpare il vostro avversario e quando no. Che vinca il migliore!!
pub fn devo_incolparlo(_me: &LogTradimento, other: &LogTradimento) -> bool {
    
    //let v: Vec<&bool> = me.azioni_passate().iter().rev().take(2).collect();
    if other.numero_veri()/75>= other.numero_falsi()/100 {
        true
    } else {
        unsafe{
            if RNG.is_none(){
                RNG = Some(ThreadRng::default());
            }
            if let Some(x) = &mut RNG{
                x.gen_bool(0.5)
            }else{
                false
            }
        }
        
    }
} 

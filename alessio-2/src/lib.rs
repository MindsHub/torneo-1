use template_torneo::LogTradimento;
use rand::{rngs::ThreadRng, Rng};

pub fn devo_incolparlo(_me: &LogTradimento, _other: &LogTradimento, rng: &mut ThreadRng) -> bool {
   
    rng.gen_bool(0.5)
        
}

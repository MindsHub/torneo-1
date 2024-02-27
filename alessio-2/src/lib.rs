use rand::{rngs::ThreadRng, Rng};
use template_torneo::LogTradimento;
static mut RNG: Option<ThreadRng> = None;
pub fn devo_incolparlo(_me: &LogTradimento, _other: &LogTradimento) -> bool {
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

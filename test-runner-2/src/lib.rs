use rand::{rngs::OsRng, Rng};
use template_torneo::LogTradimento;

pub fn test_runner_2(_me: &LogTradimento, _other: &LogTradimento) -> bool {
    OsRng.gen_bool(0.5)
}

use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{FnToImpl, LogTradimento};

///struttura per descrivere l'esecutore
pub struct MatchTradimento {
    player_a: (LogTradimento, FnToImpl),
    player_b: (LogTradimento, FnToImpl),
}

impl MatchTradimento {
    /// costruttore
    pub fn new(player_a: FnToImpl, player_b: FnToImpl)->Self{
        Self{
            player_a: (LogTradimento::new(), player_a),
            player_b: (LogTradimento::new(), player_b),
        }
    }
    ///esegue molti scontri tra individui
    pub fn compute(&mut self, n:usize)->(i64, i64){
        for _ in 0..n {
            let resp_a = self.player_a.1(&self.player_a.0, &self.player_b.0);
            let resp_b = self.player_b.1(&self.player_b.0, &self.player_a.0);
            self.player_a.0.aggiungi_azione(resp_a);
            self.player_b.0.aggiungi_azione(resp_b);
            let (pen_a, pen_b) = match (resp_a, resp_b) {
                (true, true) => (5, 5),
                (true, false) => (0, 7),
                (false, true) => (7, 0),
                (false, false) => (1, 1),
            };
            self.player_a.0.penalita += pen_a;
            self.player_b.0.penalita += pen_b;
        }
        (self.player_a.0.penalita, self.player_b.0.penalita)
    }
}




fn match_making(v: &[(String, FnToImpl)]) -> Vec<((String, FnToImpl), (String, FnToImpl))> {
    let mut x = Vec::<((String, FnToImpl), (String, FnToImpl))>::new();
    for (index, val) in v.iter().enumerate() {
        for other in v.iter().skip(index+1) {
            x.push((val.clone(), other.clone()));
        }
    }
    x
}

pub fn run_turnament(da_valutare: &[(String, FnToImpl)], n: usize)->Vec<(String, i64)>{
    let matches = match_making(da_valutare);
    //compute matches
    let v: Vec<(String, i64)> = matches
        .par_iter()
        .map(|((a_name, a_func), (b_name, b_func))| {
            let (pen_a, pen_b) =MatchTradimento::new(*a_func, *b_func).compute(n);
            vec![(a_name.clone(), pen_a), (b_name.clone(), pen_b)]
        })
        .flatten()
        .collect();
    

    //computing scoreboard
    let mut scoreboard=HashMap::<String, i64>::new();
    for (key, val) in v{
        if let Some(x) = scoreboard.get_mut(&key){
            *x+=val;
        }else{
            scoreboard.insert(key, val);
        }
    }
    let mut scoreboard: Vec<(String, i64)> = scoreboard.iter().map(|(a, b)| (a.clone(), *b)).collect();
    scoreboard.sort_by_key(|(_, x)| *x);
    scoreboard.to_owned()
}
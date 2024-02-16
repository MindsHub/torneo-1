use std::collections::HashMap;

use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use template_torneo::*;

macro_rules! register_crate {
    ($i:ident, $e:ident) => {
        $i.push((stringify!($e).to_string(), $e::$e))
    };
}

fn register_crates(da_valutare: &mut Vec<(String, FnToImpl)>){
    //aggiungi il tuo crate qui
    register_crate!(da_valutare, test_runner);
    register_crate!(da_valutare, test_runner_2);
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

fn main() {
    let mut da_valutare = Vec::<(String, FnToImpl)>::new();
    register_crates(&mut da_valutare);



    let matches = match_making(&da_valutare);
    //compute matches
    let v: Vec<(String, i64)> = matches
        .par_iter()
        .map(|((a_name, a_func), (b_name, b_func))| {
            let (pen_a, pen_b) =MatchTradimento::new(*a_func, *b_func).compute();
            vec![(a_name.clone(), pen_a), (b_name.clone(), pen_b)]
        })
        .flatten()
        .collect();
    
    let mut scoreboard=HashMap::<String, i64>::new();
    for (key, val) in v{
        if let Some(x) = scoreboard.get_mut(&key){
            *x+=val;
        }else{
            scoreboard.insert(key, val);
        }
    }
    let mut scoreboard: Vec<(&String, &i64)> = scoreboard.iter().collect();
    scoreboard.sort_by_key(|(_, x)| *x);
    for (i, (name, points)) in scoreboard.iter().enumerate(){
        println!("{}) {}: {}", i+1, name.as_str().green(), *points)
    }
}

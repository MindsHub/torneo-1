use rand::{rngs::OsRng, Rng};
use template_torneo::LogTradimento;
// /// Scegliete quando incolpare il vostro avversario e quando no. Che v
/**
# COSE DA FARE 
- vai in mona
- capire l'algoritmo dell'altro
- se l'avvesario ha risposto il 90% delle volte true:
    risponde false
-altrimenti:
    risponde true
 */
static boh: f32 = 0.0;
pub fn devo_incolparlo(me: &LogTradimento, other: &LogTradimento) -> bool {
    if other.azioni_passate().len() < 5{
        false
    }else{
        let v: Vec<bool> = other.azioni_passate().iter().rev().take(5).copied().collect();
        if v[0] == true && v[1]{
            false
        }else if v[2] == true && v[3] && v[4]{
            if let Some(true) = me.azioni_passate().last() {
                false
            }
            else{ false }
        }else{
            if other.numero_veri() > other.numero_totali()/2 && me.numero_falsi() > 1{
                return true;
            }
            else{return false;}
        }
    }
}


    /*if other.numero_totali() > 1{
        let prob = other.numero_veri() as f64/other.numero_totali() as f64;
        let res = OsRng.gen_bool(prob);
        println!("{res}");
        res
    }else{
        false
    }
*/

    /*if other.numero_veri() >  other.numero_totali()*90/100 {
        false
   }else if other.numero_falsi() > other.numero_totali()*85/100 {
        true
   }else{
        false
   }
*/



use template_torneo::*;

macro_rules! register_crate {
    ($($e:ident),*) => {
        {
            let da_valutare: Vec<(String, FnToImpl)>=vec![$((stringify!($e).to_string(), $e::devo_incolparlo)),*];
            da_valutare
        }
    };
}

fn esegui_torneo()->i64{
    //let start = Instant::now();
    let mut da_valutare=
    //aggiungi il tuo crate qua sotto: v
    register_crate!(alessio_1, alessio_2, mattiaz_2, leo_1);
    da_valutare.push(("leo_2".to_string(), devo_incolparlo));
    //calcolo classifica
    let scoreboard=runner::run_turnament(&da_valutare, 10000);
    for (nome, punteggio) in scoreboard{
        if nome == "leo_2".to_string(){
            return punteggio;
        }
    }
    return 0;
}
#[test]
fn testare(){
    println!("{}", esegui_torneo());
    panic!("aiut:(")
}
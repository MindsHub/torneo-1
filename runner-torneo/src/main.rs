use std::time::Instant;

use colored::Colorize;
use template_torneo::*;

macro_rules! register_crate {
    ($($e:ident),*) => {
        {
            let da_valutare: Vec<(String, FnToImpl)>=vec![$((stringify!($e).to_string(), $e::devo_incolparlo)),*];
            da_valutare
        }
    };
}

fn main() {
    let start = Instant::now();
    let da_valutare=
    //aggiungi il tuo crate qua sotto: v
    register_crate!(alessio_1, alessio_2, leo_2, mattiaz_2, leo_1);
    
    //calcolo classifica
    let scoreboard=runner::run_turnament(&da_valutare, 10000);

    //calcolo classifica
    let scoreboard = runner::run_turnament(&da_valutare, 10000);

    //visualizzazione classifica
    println!("Posizione Nome Punteggio");
    for (i, (name, points)) in scoreboard.iter().enumerate() {
        println!("{}) {}: {}", i + 1, name.green(), *points)
    }
    println!(
        "\nCalcolata in {:.3} secondi",
        start.elapsed().as_secs_f32()
    )
}


pub struct LogTradimento {
    azioni: Vec<bool>,
    numero_veri: usize,
    penalita: i64,
}

pub type FnToImpl = fn(&LogTradimento, &LogTradimento) -> bool;
#[allow(clippy::new_without_default)]
impl LogTradimento {
    /// quanti veri ha mandato questo giocatore?
    pub fn numero_veri(&self) -> usize {
        self.numero_veri
    }
    /// quanti falsi ha mandato questo giocatore?
    pub fn numero_falsi(&self) -> usize {
        self.azioni.len() - self.numero_veri
    }
    /// in totale quante partite sono state svolte?
    pub fn numero_totali(&self) -> usize {
        self.azioni.len()
    }
    /// costruttore (non vi serve)
    pub fn new() -> Self {
        Self {
            azioni: Vec::new(),
            numero_veri: 0,
            penalita: 0,
        }
    }
    ///descrizione dettagliata delle operazioni svolte, per algoritmi più avanzati
    pub fn azioni_passate(&self)->&Vec<bool>{
        &self.azioni
    }
    /// usata dall'esecutore
    fn aggiungi_azione(&mut self, value: bool) {
        self.azioni.push(value);
        if value {
            self.numero_veri += 1;
        }
    }
}

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
    pub fn compute(&mut self)->(i64, i64){
        for _ in 0..10000 {
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

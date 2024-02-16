pub mod runner;

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

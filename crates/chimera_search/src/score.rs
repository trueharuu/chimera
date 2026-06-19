use chimera_core::{board::Board, placement::Move, spin::Spin};

#[derive(Clone, Copy, PartialEq)]
pub struct ScoreConfig {
    pub single: usize,
    pub double: usize,
    pub triple: usize,
    pub quad: usize,
    pub spin_zero: usize,
    pub spin_mini: usize,
    pub spin_mini_single: usize,
    pub spin_single: usize,
    pub spin_mini_double: usize,
    pub spin_double: usize,
    pub spin_mini_triple: usize,
    pub spin_triple: usize,
    pub spin_mini_quad: usize,
    pub spin_quad: usize,
    pub perfect_clear: usize,
    // this proportion of the base clear score is added to the total
    pub b2b: f64,
    pub scale_b2b: bool,
    pub combo: usize,
    pub level: usize,
}

impl ScoreConfig {
    pub const fn blitz() -> Self {
        Self {
            single: 100,
            double: 300,
            triple: 500,
            quad: 800,
            spin_zero: 400,
            spin_mini: 100,
            spin_mini_single: 200,
            spin_single: 800,
            spin_double: 1200,
            spin_mini_double: 400,
            spin_triple: 1600,
            spin_mini_triple: 600,
            spin_quad: 2000,
            spin_mini_quad: 900,
            b2b: 0.5,
            scale_b2b: false,
            combo: 50,
            perfect_clear: 3500,
            level: 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ScoringEvent {
    pub board: Board,
    pub placement: Move,
    pub b2b: usize,
    pub combo: usize,
    pub config: ScoreConfig,
    pub level: usize,
}

impl ScoringEvent {
    pub fn score(mut self) -> usize {
        let line_clears_before = self.board.filled_rows().count_ones() as usize;
        self.board.apply(self.placement);
        let mask = self.board.filled_rows();
        let line_clears = mask.count_ones() as usize - line_clears_before;
        self.board.clearshift(mask);

        let base = match (line_clears, self.placement.spin()) {
            (0, Spin::None) => 0,
            (0, Spin::Mini) => self.config.spin_mini,
            (0, Spin::Full) => self.config.spin_zero,
            (1, Spin::None) => self.config.single,
            (1, Spin::Mini) => self.config.spin_mini_single,
            (1, Spin::Full) => self.config.spin_single,
            (2, Spin::None) => self.config.double,
            (2, Spin::Mini) => self.config.spin_mini_double,
            (2, Spin::Full) => self.config.spin_double,
            (3, Spin::None) => self.config.triple,
            (3, Spin::Mini) => self.config.spin_mini_triple,
            (3, Spin::Full) => self.config.spin_triple,
            (4, Spin::None) => self.config.quad,
            (4, Spin::Mini) => self.config.spin_mini_quad,
            (4, Spin::Full) => self.config.spin_quad,
            _ => unreachable!(),
        };

        let extra = if self.config.scale_b2b {
            (base as f64 * self.b2b as f64 * self.config.b2b) as usize
        } else {
            (base as f64 * (self.b2b > 0) as u8 as f64 * self.config.b2b) as usize
        };
        let combo_score = self.config.combo * self.combo;

        let pc_score = (self.board == Board::EMPTY) as usize * self.config.perfect_clear;

        let total = base + extra + combo_score + pc_score;
        
        if self.config.level != 0 {
            total * self.level * self.config.level
        } else {
            total
        }
    }
}
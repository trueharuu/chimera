use crate::spin::Spin;

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

#[derive(Clone, Copy, PartialEq)]
pub struct ScoringEvent {
    pub pc: bool,
    pub b2b: usize,
    pub combo: usize,
    pub spin: Spin,
    pub line_clears: usize,
    pub config: ScoreConfig,
    pub level: usize,
}

impl ScoringEvent {
    pub const fn score(self) -> usize {
        let base = match (self.line_clears, self.spin) {
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

        let pc_score = self.pc as usize * self.config.perfect_clear;

        let total = base + extra + combo_score + pc_score;
        
        if self.config.level != 0 {
            total * self.level * self.config.level
        } else {
            total
        }
    }
}

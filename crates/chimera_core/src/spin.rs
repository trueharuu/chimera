#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Spin {
    None,
    Mini,
    Full,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Spins {
    None,
    T,
    All,
}

impl Spin {
    pub const NB: usize = 3;
    pub const ALL: [Self; Self::NB] = [Self::None, Self::Mini, Self::Full];
}
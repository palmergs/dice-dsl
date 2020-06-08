use super::Die;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Roll {
    pub range: i64,
    pub roll: i64,
    pub modifier: i64,
    pub total: i64,
    pub keep: bool,
    pub crit: bool,
    pub bonus: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Results {
    pub rolls: Vec<Roll>,
    pub total: i64,
}
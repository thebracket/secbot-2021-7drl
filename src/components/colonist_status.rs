#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColonistStatus {
    Alive,
    StartedDead,
    DiedAfterStart,
    Rescued,
}

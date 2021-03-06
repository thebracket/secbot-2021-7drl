#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColonistStatus {
    Unknown,
    Alive,
    StartedDead,
    DiedAfterStart,
    Rescued,
}

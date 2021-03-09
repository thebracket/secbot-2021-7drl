pub enum TriggerType {
    EndGame,
    Healing,
}

pub struct TileTrigger(pub TriggerType);

pub struct TimedEvent {
    pub timer: i32,
    pub event: EventType,
}

pub enum EventType {
    Boom,
    HatchXenomorph,
}

pub enum AggroMode {
    Nearest,
    Player
}

pub struct Hostile{
    pub aggro: AggroMode,
    pub melee: Vec<Melee>,
    pub ranged: Vec<Ranged>,
}

pub struct Melee {
    pub damage: i32
}

pub struct Ranged {
    pub range: i32,
    pub power: i32,
}
pub struct Hostile {
    pub melee: Vec<Melee>,
    pub ranged: Vec<Ranged>,
}

pub struct Melee {
    pub damage: i32,
}

pub struct Ranged {
    pub power: i32,
}

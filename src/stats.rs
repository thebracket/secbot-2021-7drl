use lazy_static::*;
use std::sync::Mutex;

lazy_static! {
    static ref STATS: Mutex<PlayStats> = Mutex::new(PlayStats::new());
}

#[derive(Debug, Clone)]
pub struct PlayStats {
    pub turns_elapsed: usize,
    pub last_heard: String,
    pub total_dead: usize,
    pub total_props_smashed: usize,
    pub total_hostiles_killed: usize,
}

impl PlayStats {
    fn new() -> Self {
        PlayStats {
            turns_elapsed: 0,
            last_heard: "Nothing".to_string(),
            total_dead: 0,
            total_hostiles_killed: 0,
            total_props_smashed: 0,
        }
    }
}

pub fn reset() {
    let mut lock = STATS.lock();
    let stats = lock.as_mut().unwrap();
    stats.turns_elapsed = 0;
}

pub fn record_turn() {
    let mut lock = STATS.lock();
    let stats = lock.as_mut().unwrap();
    stats.turns_elapsed += 1;
}

pub fn record_speech<S: ToString>(s: S) {
    let mut lock = STATS.lock();
    let stats = lock.as_mut().unwrap();
    stats.last_heard = s.to_string();
}

pub fn record_death() {
    let mut lock = STATS.lock();
    let stats = lock.as_mut().unwrap();
    stats.total_dead += 1;
}

pub fn record_prop_death() {
    let mut lock = STATS.lock();
    let stats = lock.as_mut().unwrap();
    stats.total_props_smashed += 1;
}

pub fn record_monster_death() {
    let mut lock = STATS.lock();
    let stats = lock.as_mut().unwrap();
    stats.total_hostiles_killed += 1;
}

pub fn get_stats() -> PlayStats {
    let mut lock = STATS.lock();
    let stats = lock.as_mut().unwrap();
    stats.clone()
}

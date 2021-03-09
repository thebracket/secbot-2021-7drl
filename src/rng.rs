use bracket_lib::prelude::RandomNumberGenerator;
use lazy_static::*;
use std::sync::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

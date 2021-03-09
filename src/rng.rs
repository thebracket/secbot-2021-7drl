use bracket_lib::prelude::RandomNumberGenerator;
use std::sync::Mutex;
use lazy_static::*;

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}
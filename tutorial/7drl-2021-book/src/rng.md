# Adding a Global RNG

Roguelikes use a lot of random numbers, so it was a good bet that I'd need an RNG. Since I wasn't using Legion's scheduler, I didn't feel there would be much advantage to using its resources system. With hindsight, that was probably a mistake - but it worked, so I'm not grumbling too much.

`bracket-lib` includes a `RandomNumberGenerator`. It's based on xor-shift, with some ease-of-use changes applied. It has one downside: it's stateful. Generating a random number requires *mutable* access to the RNG. So it's not enough to keep it around, you have to keep it around mutably - and accessing a single RNG becomes a bottleneck whenever you use threads. "Aha", I thought! I'm not using threads, so I don't need to worry about that. 

There's definite benefits to having the RNG be a global resource; you can "seed" it and get the same results each time. Rust requires that global variables be protected by in case of concurrency. There *isn't* a way to say "I promise not to use threads, honest!" - you can't bypass the `Sync+Send` requirement without using `unsafe` code blocks. I didn't really want to do that. So, I wrapped up my RNG in a `lazy_static` (a fantastic crate that handles the boilerplate of making a safe mutable static for your program).

I also decided not to use `parking_lot` and just go with Rust's default `Mutex`. I probably should have gone with parking lot; its structures are not only faster, but they are a little easier to work with. It works with WASM. I persuaded myself that it wasn't worth the overhead for a single static - so here we are, using the default Mutex.

## Import lazy_static and Mutex

At the top of `src/main.rs`, add:

~~~rust
use lazy_static::*;
use std::sync::Mutex;
~~~

## Create a Lazy RNG

Immediately after the import statements, add the following to create a global RNG:

~~~rust
lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}
~~~

This is pretty self-explanatory if you're familiar with Rust. `RNG` is a `RandomNumberGenerator` wrapped in a `Mutex`. Mutexes are "locked" when you access them - no other thread can access it, and unlocked when you are done with it. Since there's no chance of contention, and Mutex is really fast - there's very little penalty for using this, other than some boilerplate code to access the RNG when you need it.

> If you check the real project source code, you'll see that I had an atomic variable called `REDRAW`. That was a terrible idea, and I removed it almost immediately. The idea was to limit redrawing the screen to when something needs it. Bracket-lib already does some of that, so I'm not at all sure why I thought that adding extra bookkeeping to the system was a good idea. I didn't include it in the tutorial because it was removed so early in day one that it might as well have never existed beyond a brief head-scratching moment wondering why nothing happened when I changed game sate.

## Onwards!

Next up: a little cleaning.
# SecBot - 7 Day Roguelike Challenge (2021)

[Play in your Browser](http://bfnightly.bracketproductions.com/secbot2021/). Chrome, Firefox or similar recommended. It's also published on [itch.io](https://thebracket.itch.io/secbot).

A tutorial series will follow soon.

## What is a 7DRL?

The [7-Day Roguelike Challenge](https://7drl.com/) is one of my favorite game jams. The challenge is to make a roguelike (or "roguelite" if you feel like using a more relaxed definition) in a week. It's actually a little flexible, but you have seven days to make something playable. You can start with an existing engine, re-use existing code, or start from scratch. Use whatever language you like. Finishing a game is much harder than making part of one, so it's a great discipline to work on - and it feels great to release something, even if its not likely to be the next big hit.

## What is SecBot?

I've been reading the *Murderbot Diaries* series of books by Martha Wells. Great young-adult space opera (it's been a while since I was a young adult, but still a great category). I really liked the security bot, part man, part machine. So I used the basic concept to structure the game.

The idea behind SecBot is that an outpost has ceased communications, so the morally dubious Bracket Corporation dispatch a security bot to find out what happened. Upon arrival, it becomes clear that things aren't going well for the colony - so the player rushes around collecting colonists and shepherding them back to the spaceship. I tried to bake some narrative/flavor into the game, and create a fun game you can enjoy over a coffee-break.

## How do I run this thing?

Either launch the [browser version](http://bfnightly.bracketproductions.com/secbot2021/) or download either the `.exe` file or the Linux binary. They are statically linked (written in Rust) and include everything you should need to run the game.

Alternatively, you can clone this repo and run the game with `cargo run` or `cargo run --release`.

## Updates

It isn't really in the spirit of a 7DRL to update it after the final release. I'll make a separate branch for any post-release changes (including cleanup for the accompanying tutorial posts). I'm hoping that the source and tutorial are useful to you.

## Design Constraints

In addition to the "must be created in 7 days" rule from the jam, I chose to adopt a few more constraints for this project:

* The project must work in WASM/Web Assembly and run well in a browser.
* That implies a single-threaded constraint (WASM threads work strangely), so the project doesn't use any concurrency.
* Since I'm single-threaded, there wasn't a lot of point in using Legion's scheduler. It's biggest benefit is that it relatively effortlessly adds concurrency to your game. The cost is that you wind up writing a fair amount of boilerplate (admittedly, the boilerplate is shrinking in each release) - and are stuck with a relatively rigid setup. So I ditched the scheduler completely and used a "functions, functions everywhere" approach.
* I tried to limit myself to techniques that can be gleaned from [Hands-On Rust](https://hands-on-rust.com/), my book about learning Rust and Rust Game Development. This constraint exists so that I can refer to the book when I present the game as a tutorial.

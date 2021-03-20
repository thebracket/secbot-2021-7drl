{{#include header.md}}

# Before I Started

The 7DRL rules give you a lot of flexibility. You can start with an existing code base, use engines, even use the week to improve an existing project (so long as you clearly label what you did). Given that leeway, I decided to do a bit of advanced planning.

## Tooling Decisions

The first thing I pondered was tooling. I'd largely made up my mind, so this mostly consisted of making sure that the tools I intended to use were in decent shape.

* I was sure I'd be using [bracket-lib](https://github.com/amethyst/bracket-lib). I initially put it together as a result of another Roguelike event (*/r/roguelikedev makes a Roguelike*). I used that event to learn Rust!
* I made sure my shiny new laptop had [The Gimp](https://www.gimp.org/) working well.
* I checked that my external hard-drive full of CC0 (free) graphics assets was working, and noted down a few assets (mostly from [Kenney](https://kenney.nl/)) that I thought would be useful.
* I dug into the [Rust Roguelike Tutorial](http://bfnightly.bracketproductions.com/rustbook/) to make sure that my WASM build scripts were still going to work. (I find myself in the funny position of sometimes referring back to my own work now!)
* I made sure that my working copy of [Hands-On Rust](https://hands-on-rust.com/) was on the new computer. I wrote it, I should have it all memorized, right? The truth is that while I know what it says - having spent a year writing it - I still find it comforting to be able to pull up examples.

## Some Basic Concepts

I knew I'd like to involve *Murderbot Diaries* and *Aliens* (I've been loving the Murderbot books recently!) and had a basic idea of what I wanted. I was also painfully aware that with only a week to work, I couldn't introduce more than one or two new ideas into the title. I made a couple of up-front decisions:

* I'd start with pure ASCII (well, Codepage 437), and not introduce tile graphics until the basics were functioning.
* I'd structure everything to try and make my changes visible *quickly*. I wanted to see/feel progress as I went, and be able to share it with the lovely folks on the RoguelikeDev discord.

With those decisions made, I dived into creating a minimal design document.
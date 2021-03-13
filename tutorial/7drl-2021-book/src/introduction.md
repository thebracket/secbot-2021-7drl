{{#include header.md}}

# Introduction

Every year, the "roguelikedev" community run the [7-Day Roguelike Challenge](https://7drl.com/). I don't participate in a lot of game jams, particularly really short ones; I need my beauty sleep, and a 24-hour dash to make something leaves me overly tired. A week is a decent amount of time to create a short game. Quite a few years ago, after lurking in the [RoguelikeDev](https://www.reddit.com/r/roguelikedev/) sub-reddit for a while, I decided to jump in. I released "Tech Support - The Roguelike". It wasn't very good, but I had fun making it - and that's the key. You don't have to make something amazing, you just have to enjoy making something! I've participated in a few since then, and feel it's a worthy challenge for any aspiring game developer.

This year, I set out to make *SecBot*. I wanted to make a fun "coffee-break" length game. I'd been reading the *Murderbot Diaries* series of novels, and re-watched *Aliens* - so I felt like putting some elements from those into the game. I also wanted to impose a few more constraints upon myself:

* The game must run in a web browser with WASM.
* Because of this, it must be single-threaded.
* The single-threaded requirement lead to "why bother with the Legion scheduler and formal systems?" They are a lot of boilerplate, and in a regular game give you some *amazing* performance and automatic concurrency. Since I can't use concurrency, why bother with the boilerplate and rigidity of a systems-based setup? I went with functions, instead - and used Legion as a data-store. It's great for that, you can query it easily whenever you want to.
* Since I was writing in a hurry, things aren't as polished or smooth as I'd normally require.

# What does this series cover?

Thinking about it, I really had two options for writing this series. I could either present the finished product and explain all of the sections (which would be a good exposition, but dry), or I could give you a blow-by-blow from the gamedev trenches. I concluded that the latter was more palatable and fun. This tutorial will walk you through each section of progress, in the order it occurred. You'll see bugs, code land-mines, and concern as things just don't work. You'll also see joy when things come together, know how badly I was sweating when serious bugs appeared with only a few hours to go, and understand the process that let me finish a game in a little under a week.

# Acknowledgements

I'd like to thank [-Mel.](https://melwolverson.com/), my ever-patient wife for letting me work late on this project. I'd also like to thank Tammy at PragProg (my book publisher) for letting me ease-off of writing a bit (Hands-On Rust is on its way to final production, and a new title is in the works) to pursue this. As ever, my coworkers at iZones have been awesome about letting me take the time to do side-projects.

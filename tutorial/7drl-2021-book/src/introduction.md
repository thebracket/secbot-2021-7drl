{{#include header.md}}

# Introduction

Game jams have become a significant part of the gamedev community. They offer some great opportunities to meet with people, collaborate if you want to, try out new ideas, and hone your skills. They're more like a group of musicians jamming than a formalized development project; you never quite know what'll come out at the end. Despite the [ever growing list of jams](https://itch.io/jams), I'm a bit picky about which jams I join. I try to find ones that fit my interests, last long enough to make something without losing too much sleep (as I get older, sleep has become more important!). I also prefer ones that have encouraging communities and aren't hyper-competitive.

My favorite jam is the [7-Day Roguelike Challenge](https://7drl.com/).  Quite a few years ago, after lurking in the [RoguelikeDev](https://www.reddit.com/r/roguelikedev/) sub-reddit for a while, I decided to jump in. I released "Tech Support - The Roguelike". It wasn't very good, but I had fun making it - and that's the key. I followed that up with *Dankest Dungeons*, a web-based game in which you designed dungeons or played dungeons designed by other players. Neither are likely to win any prizes, but I enjoyed the feeling of having finished something, learned a lot along the way, and chatted with some awesome people.

This year, I set out to make *SecBot*. I wanted to make a fun "coffee-break" length game - something you could enjoy in a short burst.

## What does this series cover?

Writing a tutorial series about a 7-day game jam is interesting. There really were two options: I could polish everything up and give you a tutorial on how to make the game I created, or I could give you a blow-by-blow from the trenches. I concluded that the latter was more palatable and fun - even if it sometimes feels like I'm showing you my dirty laundry. You'll see bugs, code land-mines, and consternation when things didn't work. You'll also see the joy when the things came together, the sweat when bugs are biting with only a few hours to go, and my slight confusion as to how to actually submit the finished project. It won't be meticulously written good code, but hopefully it will give you a good idea of how jam games come together - and how following a plan can see you across the finish line.

## Constraints (Self-Imposed)

In addition to the "you must finish in 7 days", I added a couple more constraints to my development:

* The game must run in a web browser with WASM.
* Because of this, it must be single-threaded.
* The single-threaded requirement lead to "why bother with the Legion scheduler and formal systems?" They are a lot of boilerplate, and in a regular game give you some *amazing* performance and automatic concurrency. Since I can't use concurrency, why bother with the boilerplate and rigidity of a systems-based setup? I went with functions, instead - and used Legion as a data-store. It's great for that, you can query it easily whenever you want to.

These constraints can be boiled down to "must run in a browser" - which felt important, the web is the lingua franca of the Internet and I didn't have time to test something on every platform out there.

## Acknowledgements

I'd like to thank [-Mel.](https://melwolverson.com/), my ever-patient wife for letting me work late on this project. I'd also like to thank Tammy at PragProg (my book publisher) for letting me ease-off of writing a bit (Hands-On Rust is on its way to final production, and a new title is in the works) to pursue this. As ever, my coworkers at iZones have been awesome about letting me take the time to do side-projects.

## Onwards!

I'm hoping that this tutorial is useful to you. If you'd rather just dive straight into the finished product, [the source is up on Github](https://github.com/thebracket/secbot-2021-7drl). So let's warp backwards through time a little to the week before the 7-day Roguelike Challenge (7DRL) begun.
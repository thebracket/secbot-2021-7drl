{{#include header.md}}

# Initial Design

I'm a big proponent of scribbing out some basic design notes before you start, so I did this ahead of time. I used a template similar to that found in *Hands-On Rust*, in particular focusing on the "minimum viable product" (enough of a game that I wouldn't be ashamed to share it) and lots of "stretch goals". I didn't meet all of the stretch goals - and that's ok. It's a timed game jam, there's only so much I could squeeze in! I do think it's important to have a plan when you start a time trial - and its important to structure it so that you can see regular progress. There's nothing worse than trying to implement all of a giant design and not seeing much functionality until near the end - inevitably accompanied by a panic as you realize what doesn't work, or worse - the game isn't fun to play.

Here are my design notes, following the template I presented in my book.

## Project Name

**SecBot**. I originally went with *Murderbot*, but I didn't want to infringe upon copyrights.

## Short Description

A coffee-break Roguelike with 4 procedurally generated levels. Rescue colonists, fight monsters, and explore the dungeon. Emphasis on ranged combat. 4-way turn-based movement on a gridded map.

## Story

SecBot is a human/robot hybrid. Employed by *Bracket Corporation*, he is sent to a mining outpost that has ceased communicating with its parent company. Upon arrival, it becomes clear that things have gone horribly wrong. Colonists beg for help, and flee to SecBot's ship. Nasty aliens crawl around the base, threatening both SecBot and the colonists. SecBot searches the colony, battles the Alien Queen, and saves the day.

## Basic Game Loops

Primary loop:

1. Enter dungeon level.
2. Explore, revealing the map and activating entities.
    1. Encounter enemies and battle them.
    2. Encounter colonists who greet the player and flee to their space ship.
3. Find things like healing stations.
4. Locate the exit to the next level, and go to 1.
5. At any time, SecBot can choose to fly away. Report the rescue progress.

## Minimum Viable Product

* Create 4 basic dungeon map levels.
     * Top-level, the colony on the surface.
     * Mine top, the beginning of the mine with rooms around it.
     * Mine center, a mine shaft surrounded by natural looking cavern.
     * Cavern, holding alien eggs and the queen.
* Place the player and let them walk around.
* Make doors automatic.
* Spawn colonists.
* Colonists activate when the player sees them.
* Active colonists path to the game exit, including across map levels.
* Spawn monsters.
* Monsters inflict melee damage.
* Player can shoot monsters.
* Monsters can shoot back if they have a ranged attack.
* Monsters path towards the player, killing any colonists or player they can see.
* End-game screen for dying.
* End-game screen for leaving via the space ship.
* Score and progress display.

## Stretch Goals

* Colonists can talk to you, giving the game flavor.
* Spawn props to make it feel like a living colony.
* Rooms with themed content, to give some consistency to the randomness.
* Tiles that heal you.
* Monster variety.
* Explosions.
* Timed-explosions for grenades.
* A nice, complicated projectile sytem.
* End the game by shooting out windows.
* Colonists with weapons who fight back.
* Another SecBot who helps you.
* A really pretty looking planetary surface.
* Something better than hit points for representing health.
* Weaponry variety.
* Eggs that turn into monsters after a count-down.
* Fun rooms like a colonist shouting "game over" and dropping a grenade. Should be a way to save them.

# Wrap-Up

That's quite the list! I didn't achieve all of the stretch goals, but most of them made it in. I'm glad I made a plan up-front; I referred to it a lot during development, and scratching items off of the list was very satisfying. I'm also glad I didn't write a huge, super-detailed design document. Particularly in a game jam, life happens - something doesn't work as expected, and you wind up writing some odd code to patch around it.

Up next: a starting template.

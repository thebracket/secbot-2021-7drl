# Vertical Navigation

As day three drew to a close, I decided to make a start on getting staircases working. It won't be completed on day 3, but it's a good start.

## Command Placeholders

A good start to allowing SecBot to climb/descend stairs is to support the commands.

Open up `src/game/player.rs` and look at the `player_turn` function. There's a large `match` statement handling possible key-presses. You want to add two more commands to the list:

~~~rust
VirtualKeyCode::Comma => go_up(ecs, map),
VirtualKeyCode::Period => go_down(ecs, map),
~~~

You haven't written these functions yet, so it won't compile - but now you have the skeleton of making the comma key (`<` with shift) indicate a desire to go up, and `>` (period/full stop) indicate a desire to go down.

Let's drop-in some placeholder commands to allow the game to compile. Add two new functions to the `player.rs` file:

~~~rust
fn go_up(ecs: &mut World, map: &mut Map) -> NewState {
    NewState::Wait
}

fn go_down(ecs: &mut World, map: &mut Map) -> NewState {
    NewState::Wait
}
~~~

## Update the Instructions

It's always a good idea to give your player some idea of how to play the game. When we setup the WASM build, we included some instructions in the HTML. Now that we've added some commands, let's include them in the instructions. Open `wasm_help/index.html`. Add one more sentence to the instruction text:

~~~html
<p style="color: #55ff55; font-family: 'Courier New', Courier, monospace; font-size: 10pt;">&lt; and &gt; go up and down levels if you are on an appropriate staircase.</p>
~~~

Now that we're able to catch the up and down commands, and have told the player what to do - let's start making them do something.

## Implementing Up Stairs

Open `src/map/tile.rs`. In the `TileType` enum, we want to add one more type of tile:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Capsule,
    Wall,
    Floor,
    Outside,
    StairsDown,
    StairsUp,
}
```

We also need to make a function to build a tile of this type. In the same file, add the following builder function:

```rust
pub fn stairs_up() -> Self {
        Self {
            glyph: to_cp437('<'),
            color: ColorPair::new(YELLOW, BLACK),
            blocked: false,
            opaque: false,
            tile_type: TileType::StairsUp,
        }
    }
```

Finally, we need to make `TileType` public---we're using it outside of the map module, now. Open `src/map/mod.rs` and add one line:

```rust
pub use tile:TileTYpe;
```

## Finding Stairs

Up and Down commands only work when the player (or other entity that uses stairs) is standing on a stair-case. That means we need a quick way to find out where the stairs are for a level. Open `src/map/layer.rs`. Add `TileType` to the list of types you are importing from `super`:

~~~rust
use super::{layerbuilder::*, Tile, TileType, HEIGHT, TILES, WIDTH};
~~~

The layer now knows what to do with the `TileType` type. Finding the downward staircase map location can be done with an iterator call. Add the following function to the `layer.rs` file, as an implemented method for `Layer`:

~~~rust
impl Layer {
    ...
    pub fn find_down_stairs(&self) -> Point {
        let idx = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| t.tile_type == TileType::StairsDown)
            .map(|(idx, _)| idx)
            .nth(0)
            .unwrap();
        self.index_to_point2d(idx)
    }
}
~~~

This works by iterating through the `tiles` list, adding an enumeration (for the tile index). It filters the iterator, only accepting down staircases. It then takes the first occurrence, and transforms the result to a `Point` listing the staircases' map location. It would be more efficient to calculate this once and cache it - but the speed benefits are negligible, so I stuck with this method.

## Setting the Current Layer

We also need a way to change the current in-play layer. Open `src/map/map.rs` and add one more function to the `Map` implementation:

```rust
pub fn set_current_layer(&mut self, new_layer: usize) {
    self.current_layer = new_layer;
}
```

You can call this function when the player changes layer, and since we're already tracking the current layer---the game will move to rendering the current map level.

## Player Movement

We need a few extra mechanisms to support player movement between levels.

### Going Up

In `src/game/player.rs`, we can now flesh out the `go_up` function. We want to check that the player is actually standing on an up staircase, and if they are change their position to the upwards-level's down staircase---and update their map position. Flesh out the function as follows:

```rust
fn go_up(ecs: &mut World, map: &mut Map) -> NewState {
    let mut find_player = <(&Player, &mut Position)>::query();
    find_player.for_each_mut(ecs, |(_, pos)| {
        let idx = map.get_current().point2d_to_index(pos.pt);
        if map.get_current().tiles[idx].tile_type == TileType::StairsUp {
            // It really is an up staircase
            let new_layer = pos.layer - 1;
            map.set_current_layer(new_layer as usize);
            pos.layer = new_layer;
            pos.pt = map.get_current().find_down_stairs();
        }
    });
    NewState::Player
}
```

### Going Down

We can do the same for the `go_down` function stub in `player.rs`. We check that the player is standing on a down staircase, and update the player's position to the layer's starting point:

```rust
fn go_down(ecs: &mut World, map: &mut Map) -> NewState {
    let mut find_player = <(&Player, &mut Position)>::query();
    find_player.for_each_mut(ecs, |(_, pos)| {
        let idx = map.get_current().point2d_to_index(pos.pt);
        if map.get_current().tiles[idx].tile_type == TileType::StairsDown {
            // It really is a down staircase
            let new_layer = pos.layer + 1;
            map.set_current_layer(new_layer as usize);
            pos.layer = new_layer;
            pos.pt = map.get_current().starting_point;
        }
    });
    NewState::Player
}
```

### Turn Structure Adjustments

We're going to subtly change the turn structure, to give the game a chance to spend a tick processing player instructions before moving on to other tasks. Our basic flow will become:

`Waiting -> PlayerTurn -> EnemyTurn -> Waiting`

Open `src/main.rs` and adjust `TurnState` to include a new `PlayerTurn` entry:

```rust
enum TurnState {
    WaitingForInput,
    PlayerTurn,
    EnemyTurn,
    Modal { title: String, body: String },
    GameOverLeft,
    ...
```

Since we're doing a dance of returning a `NewState` and using it to set the `TurnState` at the right time, you also need to add a `Player` entry to `NewState` in the same file:

```rust
pub enum NewState {
    NoChange,
    Wait,
    Player,
    Enemy,
    LeftMap,
}
```

Now scroll down to where we are handling turn states in `main.rs`. You want to add two lines (the program won't compile until you do):

```rust
                NewState::Wait
            }
            TurnState::GameOverLeft => render::game_over_left(ctx),
+           TurnState::PlayerTurn => NewState::Enemy, // Placeholder
        };
        match new_state {
            NewState::NoChange => {}
            NewState::Wait => self.turn = TurnState::WaitingForInput,
            NewState::Enemy => self.turn = TurnState::EnemyTurn,
            NewState::LeftMap => self.turn = TurnState::GameOverLeft,
+           NewState::Player => self.turn = TurnState::PlayerTurn,
        }
    }
}
```

Now when it's the player's turn, we switch to `Enemy` mode, and if its `Player` time in `NewState` we set the player appropriately.

The game is coming along nicely! You can now find up/down staircases and use them to change level, allowing you to visit the whole map. You may run into issues with the stairs being obscured by a monster---but we'll worry about that on day 4.

## Wrap-Up

That concludes day 3's development. It mostly focused on map generation, with a bit of gameplay (to start using levels) thrown in. As day 4 approached, I was starting to feel the pressure - so we'll dive into a smorgasboard of game changes.

> You can find the source code for `hello_modal` [here](https://github.com/thebracket/secbot-2021-7drl/tree/tutorial/tutorial/stairs1/).

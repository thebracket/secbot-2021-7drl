# Starting Template and Build

At the very start of the jam, I grabbed a handy template I like to keep around for basic ASCII/CodePage 437 based roguelikes. I wound up modifying it a bit to fit the "no systems, no threads" constraints - but the basic template lets me get started quickly. This section will go over the template and how it got me started.

## Creating a project and building dependencies

I started the project by finding my home directory, and running `cargo init secbot`. This creates the usual "Hello, World" command-line program and makes a basic `Cargo.toml` file. Very basic stuff, but a necessary start.

I then opened up `Cargo.toml` and added in the dependencies I knew I'd need, set the project name, and cleaned up the default comments. `Cargo.toml` looks like this, now:

```toml
[package]
name = "secbot"
version = "0.1.0"
authors = ["Herbert Wolverson <herberticus@gmail.com>"]
edition = "2018"

[dependencies]
bracket-lib = { git = "https://github.com/amethyst/bracket-lib.git" }
legion = { version = "0.3.1", default-features = false, features = ["wasm-bindgen"] }
lazy_static = "1.4.0"
```

> If you read the early commits in the repo, you'll notice that I goofed and committed a local path to my `bracket-lib` source code rather than the Git repo. The two are the same, and I've fixed it in the tutorial. If you're wondering why I used the git version rather than the published crate, it's because of a bug in random number generation in WASM. I have a fix for this ready to go, but didn't have time to publish the crate before the 7-day challenge started.

## Hello, Bracket-lib!

Next up was opening `src/main.rs` and pasting in "Hello, Bracket" from the *Flappy Dragon* chapter of my book. I've written this so many times now that I can do it in my sleep; one of the perks of writing a book and the library it uses. The "hello bracket" source looks like this:

~~~rust
use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("SecBot")
        .build()?;

    main_loop(context, State{})
}
~~~

Ok, so that's not very exicting. It gets me a console window on the screen, and `Hello, Bracket Terminal!` in white on black. It's a necessary start.

## WASM Building

I knew from the start that I wanted to support Web Assembly as a target. `Bracket-lib` WASM builds require a tool called `wasm-bindgen`, so I made sure that was installed by typing: `cargo install wasm-bindgen`. It takes a while to compile, time for coffee!

Once that was in place, I pulled up a template Windows batch file I use for this:

~~~batch
@ECHO OFF
cargo build --target wasm32-unknown-unknown --release

wasm-bindgen .\target\wasm32-unknown-unknown\release\secbot.wasm --out-dir .\wasm_help\staging --no-modules --no-typescript
copy .\wasm_help\index.html .\wasm_help\staging\index.html

REM Send to server. Not included on Github so I'm not giving you server details. Sorry.
./webglbuild2.bat
~~~

The file `webglbuild2.bat` is excluded from Github so I don't give you access to my server. It's pretty simple: it copies the `wasm_help\staging` directory to the deployment folder on my server.

> Note that you need a web server to serve up your WASM build. Chrome and Firefox *really* don't like serving WASM builds from a `file://` path for security reasons.

If you're using a platform other than Windows, the commands are the same - just replace `copy` with `cp` and change `@ECHO OFF` to `#/bin/bash` or whatever your platform needs.

Anyway, before this will work you need some helpers. Create a new folder called `wasm_help`. Inside that folder, make a `staging` directory - this will hold the build to send to the server. You also need to put an `index.html` file into your `wasm_help` folder.

The final structure looks like this:

* project folder
    * src
    * target
    * wasm_help
        * staging
        * index.html
    * Cargo.toml

The contents of the `index.html` file are:

~~~html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  </head>
  <body style="background-color: black;">
    <h1 style="color: white; font-family: 'Courier New', Courier, monospace; font-size: 10pt;">SecBot (2021 7DRL) - by Herbert Wolverson</h1>
    <canvas id="canvas" width="896" height="496"></canvas>
    <script src="./secbot.js"></script>
    <script>
      window.addEventListener("load", async () => {
        await wasm_bindgen("./secbot_bg.wasm");
      });
    </script>
  </body>
</html>
~~~

As you can tell, I'm not great at HTML/CSS. This is designed to be the bare minimum: it creates a canvas, loads the `wasm` file and runs it. It's derived from the various `wasm-bindgen` tutorials out there.

> Why 896 by 496 for the canvas? I'd decided on a 112x62 console (8x8 font). So I went with the natural size from there. You'll see this in a moment.

# Building the superstructure

In my mental design sketches, the map was going to be 80 tiles wide by 60 characters tall. That's probably larger than I actually needed, but it worked. I needed room for a UI, so I went with `112x62` for my initial window size. I opened up `src/main.rs` and modified the initializer:

~~~rust
fn main() -> BError {
    let context = BTermBuilder::simple(112, 62)?
        .with_title("Secbot - 2021 7DRL")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
~~~

Notice that:

* I've modified `main` to return a `BError`, just like in *Hands-on Rust*. This lets me use the question mark operator rather than throwing `expect` everywhere.
* I added a window title.
* I capped the frame rate at 30 FPS. This keeps the game from eating too much CPU, and gives a consistent render speed.
* I've added a `new` function to `State`. We'll get there in a second.

This is a pretty tried-and-true setup, so testing consisted of `cargo run` - yup, it works.

## Implementing State and Initializing Legion

I was sure that I'd be using Legion, so I extended the `use` statements at the top of `main.rs` to include it:

~~~rust
use bracket_lib::prelude::*;
pub use legion::*;
~~~

I then extended `State` to include an Entity-Component System world:

~~~rust
struct State {
    ecs: World,
}
~~~

Finally, I added a `new` function to act as a constructor for `State`:

~~~
impl State {
    fn new() -> Self {
        { ecs: World::default(), map }
    }
}
~~~

Once again, a quick `cargo run` was enough to see that it didn't explode.

## Finding the Map

With a pretty solid idea for how the map should work, it was clear that I'd need one. I extended the `use` statements in `main.rs` to include one:

~~~
use map::Map;
~~~

Then I created a directory called `map` (`src` is the parent) and made a file called `mod.rs`. I like to keep my `mod.rs` files relatively clean - mostly just importing other things and setting module-wide constants. The `mod.rs` files looks like this:

~~~rust
pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 60;
const TILES: usize = WIDTH * HEIGHT;
pub const NUM_LAYERS: usize = 5;

mod tile;
use tile::Tile;
mod layer;
use layer::Layer;
mod map;
pub use map::Map;
//mod layerbuilder;
~~~

The top part is pretty self-explanatory: it sets the `WIDTH` and `HEIGHT` constants to the map dimensions. It calculates `TILES` to be the number of tiles this requries (80x60 = 4,800). These are constants to make it easy to change them if I change my mind on some design elements later on.

The rest refers to a bunch of modules we haven't created yet! I had a good idea of what I wanted (I've used this template before), so it served as a signpost for development. It won't compile at this point.

> Notice that `LAYERS` is equal to 5. It really should have read `4`, but I missed it when I was setting this up. I've left the bug in place so that you can see the progression of development under a time crunch.

### Making Tiles

My map is going to be tile-based, so a good starting point was "what is a tile?". In the `map` directory, I created a file named `tile.rs` and created a `Tile` structure:

~~~rust
use bracket_lib::prelude::*;

#[derive(Clone)]
pub struct Tile {
    pub glyph: FontCharType,
    pub color: ColorPair,
    pub blocked: bool,
    pub opaque: bool,
}

impl Tile {
    pub fn default() -> Self {
        Self {
            glyph: to_cp437('.'),
            color: ColorPair::new(GREY, BLACK),
            blocked: false,
            opaque: false,
        }
    }
}
~~~

That's pretty much the minimum for a tile:

* `glyph` tells the game what codepage-437 character to render for the tile.
* `color` defines a foreground and background color.
* `blocked` and `opaque` will be used when movement and field-of-view come into play. If a tile is `blocked`, you can't walk into it. If its `opaque`, you can't see through it.

### Layering the Cake

I'd decided up-front that I was going to have multiple levels, and entities other than the player needed to be able to navigate them. That required that I have *all* the map layers available when the world was created - I couldn't lazily make them as needed. I also knew that the overall game map would consist of several layers (4, even though I wrote 5 in the definition file!). So I created a `layer.rs` file in the `map` directory and added in a basic description of a `Layer` type:

~~~rust
use super::{Tile, HEIGHT, TILES, WIDTH};
use bracket_lib::prelude::*;
use legion::*;

pub struct Layer {
    pub tiles: Vec<Tile>,
    pub starting_point: Point,
}
~~~

I haven't written `layerbuilder` yet, but it's coming. We'll get to that in a second. Otherwise, the layer is pretty simple: a vector of `Tile` types, and a `Point` defining where the player starts on the level. I wanted some functionality, so I started implementing things for `Layer`. First up, a constructor:

~~~rust
impl Layer {
    pub fn new(depth: usize, ecs: &mut World) -> Self {
        let layer = match depth {
            _ => Self {
                tiles: vec![Tile::default(); TILES],
                starting_point: Point::new(WIDTH / 2, HEIGHT / 2),
            },
        };
        layer
    }
}
~~~

This is a little odd at first glance. It takes the `depth` (layer number) and a mutable reference to the ECS as parameters (so we can add stuff to the game when we build the map). It just makes an empty level with no entities on it (you'll get a warning for not using the `ecs` at this point).

I also wanted some rendering code. Note that I'm offsetting all the positions by 1 - I wanted to put a border around the map. Here's the `render` function; it should look familar, it's *very* similar to that found in *Hands-On Rust*:

~~~rust
impl Layer {
    // The `new` function goes here

    pub fn render(&self, ctx: &mut BTerm) {
        let mut y = 0;
        let mut idx = 0;
        while y < HEIGHT {
            for x in 0..WIDTH {
                let t = &self.tiles[idx];
                ctx.set(x+1, y+1, t.color.fg, t.color.bg, t.glyph);
                idx += 1;
            }
            y += 1;
        }
    }
}
~~~

It iterates the map, and draws each tile. Very simple stuff.

### The Map - A structure of layers

The map is a collection of layers, with some helpers to access it. Create a new file, `map.rs` inside the `map` directory. The basic structure is:

~~~rust
use super::{Layer, NUM_LAYERS};
use bracket_lib::prelude::*;
use legion::World;

pub struct Map {
    pub current_layer: usize,
    layers: Vec<Layer>,
}
~~~

So there's an index to the currently active layer, and a vector of `Layer` types. Now, let's implement a constructor for it:

~~~rust
impl Map {
    pub fn new(ecs: &mut World) -> Self {
        let mut layers = Vec::with_capacity(NUM_LAYERS);
        for i in 0..NUM_LAYERS {
            layers.push(Layer::new(i, ecs));
        }
        Self {
            current_layer: 0,
            layers,
        }
    }
~~~

> Note that the implementation continues, keep adding to the `impl` block. 

The constructor creates a vector with capacity for the number of layers we defined in `mod.rs`. It then iterates from 0 to the number of layers, pushing a new layer - and passing in the layer number and the ECS.

I wanted a quick way to render the current layer, so the next implemented function is `render`:

~~~rust
    pub fn render(&self, ctx: &mut BTerm) {
        self.layers[self.current_layer].render(ctx);
    }
~~~

Very straightforward - it just calls `render` for the current map layer. I also needed to be able to access the individual layers:

~~~rust
    pub fn get_current(&self) -> &Layer {
        &self.layers[self.current_layer]
    }

    pub fn get_current_mut(&mut self) -> &mut Layer {
        &mut self.layers[self.current_layer]
    }
}
~~~

These just return a pointer to the requested layer.

## Minimal map drawing

Now that the map exists (albeit as a set of empty maps, consisting of just floors), we can update the `src/main.rs` function to use it. Start by adding to the `main.rs` include list:

~~~rust
use bracket_lib::prelude::*;
pub use legion::*;
pub mod map;
pub use map::*;
~~~

Then extend `State` to hold a map and initialize it:

~~~rust
struct State {
    ecs: World,
    map: map::Map,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let map = map::Map::new(&mut ecs);
        Self { ecs, map }
    }
}
~~~

### Drawing the Map

I adjusted the `tick` function in `main.rs` to render the map and draw a border around it:

~~~rust
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
            ctx.cls();
            use map::{HEIGHT, WIDTH};
            ctx.draw_hollow_box(0, 0, WIDTH+1, HEIGHT+1, GRAY, BLACK);
            ctx.print_color(2, 0, WHITE, BLACK, "┤ SecBot 2021 7DRL ├");
            ctx.draw_hollow_box(WIDTH+1, 0, 30, HEIGHT+1, GRAY, BLACK);
            ctx.set(WIDTH+1, 0, GRAY, BLACK, to_cp437('┬'));
            ctx.set(WIDTH+1, HEIGHT+1, GRAY, BLACK, to_cp437('┴'));
            self.map.render(ctx);
        }
    }
}
~~~

You can run the game now, and see a field of `.` characters. Map rendering is working!

# Next-Up: Entities

That's not the most impressive game ever, but getting a field of dots onto the console is a great start.
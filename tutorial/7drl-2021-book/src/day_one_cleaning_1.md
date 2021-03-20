{{#include header.md}}

# Cleaning Up the Sandbox

The template was written really quickly, and that tends to result in poor code choices. A couple of things bugged me, so I took a moment to clean them up.

## Creating a Render Module

I didn't like having my render code mixed in with my turn state in `main.rs`. So I created a new module called `render`. To do this:

1. Create a new directory, `src/render`.
2. Create a new file, `src/render/mod.rs`.
3. In the imports in `main.rs`, add `mod render;`.

This leaves the following directory structure:

* src
    * components
        * `description.rs`
        * `glyph.rs`
        * `mod.rs`
        * `position.rs`
        * `tags.rs`
    * map
        * layerbuilder
            * `mod.rs`
            * `entrance.rs`
        * `layer.rs`
        * `map.rs`
        * `mod.rs`
        * `tile.rs`
    * render
        * `mod.rs`
    * `main.rs`
* wasm_help
    * `index.html`
* `Cargo.toml`

## Populating the Render Module

In the `render/mod.rs` file, I started out with some imports:

~~~rust
use bracket_lib::prelude::*;
use legion::*;
use crate::map::{ Map, WIDTH, HEIGHT };
use crate::components::{Position, Glyph};
~~~

I actually cheated a bit, and let `rust-analyzer` help me with this. I then copied the `render_glyphs` function from `main.rs` and turned it into a stand-alone function in `render/mod.rs`:

~~~rust
pub fn render_glyphs(ctx: &mut BTerm, ecs: &World, map: &Map) {
    let mut query = <(&Position, &Glyph)>::query();
    query.for_each(ecs, |(pos, glyph)| {
        if pos.layer == map.current_layer as u32 {
            ctx.set(
                pos.pt.x + 1,
                pos.pt.y + 1,
                glyph.color.fg,
                glyph.color.bg,
                glyph.glyph,
            );
        }
    });
}
~~~

No real changes, other than it doesn't access `self` - and takes the ECS and map as parameters.

I then grabbed the ugly box drawing code from `main.rs` and put it into another stand-alone function:

~~~rust
pub fn render_ui_skeleton(ctx: &mut BTerm) {
    ctx.draw_hollow_box(0, 0, WIDTH+1, HEIGHT+1, GRAY, BLACK);
    ctx.print_color(2, 0, WHITE, BLACK, "┤ SecBot 2021 7DRL ├");
    ctx.draw_hollow_box(WIDTH+1, 0, 30, HEIGHT+1, GRAY, BLACK);
    ctx.set(WIDTH+1, 0, GRAY, BLACK, to_cp437('┬'));
    ctx.set(WIDTH+1, HEIGHT+1, GRAY, BLACK, to_cp437('┴'));
}
~~~

## Updating the Main File

I opened up `main.rs` and cleaned it up to use these functions rather than containing the logic itself. The `tick` function became:

~~~rust
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        render::render_ui_skeleton(ctx);
        self.map.render(ctx);
        render::render_glyphs(ctx, &self.ecs, &self.map);
    }
}
~~~

Then delete the `render_glyphs` function from `main.rs`.

This doesn't change *any* functionality, so I didn't include an example or screenshot for it.

## Adding a LICENSE and README

I went ahead and added a `LICENSE` file to the root of the project. It's the standard MIT license, allowing you to do whatever you like with the code. I also created a minimal `README.md` file for the Github front page:

~~~markdown
# SecBot - 7 Day Roguelike Challenge (2021)

This is my 7DRL entry. I'll keep adding to it here as I work on it. I'll keep a playable [WASM/WebGL Version](http://bfnightly.bracketproductions.com/secbot2021/) updated as well.
~~~

I can my `webglbuild.bat` file, and uploaded the resulting minimal program to my server - and tested that I had a working program in WASM land.

## Pushing to Github

Finally, I connected my local repo to the [Github Repo](https://github.com/thebracket/secbot-2021-7drl/) I'd made for the project and pushed everything upstream.

## Onwards!

With the cleaning done, it was time to add some turn state and modal rendering.
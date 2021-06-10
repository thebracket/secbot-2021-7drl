# Active/Idle Colonists

I wasn't very happy with the way colonists were being activated---and needed a way to also activate monsters. The logical way to do this seemed to be to add an `Active` tag component. Thinking further about it, i realized that a `CanBeActivated` tag component would also be helpful---allowing me to make the code for activating an entity when it is discovered a bit more generic. In this section, we'll switch away from the `Unknown` colonist status and start using ECS tags to keep track of state.

> Legion makes it expensive to rearrange tags, but not *that* expensive. You aren't activating things all that often in the grand scheme of things, so the performance penalty when Legion rearranges archetypes because of an insertion isn't really a problem for this game. If the game were changing states a lot, we'd probably use an `Option` component.

## Defining the New Tags

The new tag components are like other tags---just an empty struct. Open `src/components/tags.rs` and add the new components:

```rust
pub struct Active;

pub struct CanBeActivated;
```

That's straightforward enough. Now open `src/components/colonist_status.rs` and remove the `Unknown` enumeration option:

```rust
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColonistStatus {
    Alive,
    StartedDead,
```

The game won't compile yet, now we need to start adjusting the systems that consume the components.

## Adjusting Colonist Spawn

The first thing to do is change the colonist spawns to include the new `CanBeActivated` component. Open `src/map/layerbuilder/colonists.rs` and change the `spawn_random_colonist` function to list `ColonistStatus` as `Alive` and include the new component.

One of the few things that really bugs me about Legion is that once you try and add too many components at a time, the World's `push` function stops working. Rust really needs proper variadics, so I understand why this is the case (Legion only implemented so many template interfaces)---but it would be nice if a macro were in place to work around it. Anyway, we're planning on having a lot of varied colonists. That means it's time to split colonist spawning into a "base colonist" (with the minimum required to be a colonist) and functions for spawning individual colonists. The idea is that as we add rooms/levels, we'll add in some variable behavior.

Open `/src/map/layerbuilder/colonists.rs` and add a new function:

```rust
fn build_base_colonist(ecs: &mut World, location: Point, layer: u32) -> Entity {
    ecs.push((
        Colonist { path: None },
        Position::with_pt(location, layer),
        Glyph{ glyph: to_cp437('☺'), color: ColorPair::new(LIME_GREEN, BLACK) },
        Description("A squishy friend. You are here to rescue your squishies.".to_string()),
        ColonistStatus::Alive,
        Name("Colonist".to_string()),
        Targetable {},
        CanBeActivated {},
    ))
}
```

Notice how `build_base_colonist` return an `Entity`. We'll call it as a base, and then use Legion's command system to modify the individual's components as-needed. That's not the fastest code in the world, but we call it so rarely that it doesn't matter.

Now we update `spawn_random_colonist` to use the new pattern:

```rust
pub fn spawn_random_colonist(ecs: &mut World, location: Point, layer: u32) {
    // Using this pattern because Legion has a limit to how many components it takes in a push
    let entity = build_base_colonist(ecs, location, layer);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec!["Thanks, SecBot!".to_string()],
        },
    );
    commands.flush(ecs);
}
```

First, the function retrieves a new entity from `build_base_colonist`. Then it constructs a `CommandBuffer`---a set of commands for Legion to execute in a batch. It adds a `Dialog` component for the random colonist, and flushes the commands. This builds the colonist, and then gives them dialog to present to the player.

We want the first colonist to have different dialog. They act as an introduction, so its our chance to add a bit of flavor to the game. Update the `spawn_first_colonist` function as follows. Notice how the function is following the same pattern, but adding different dialog:

```rust
pub fn spawn_first_colonist(ecs: &mut World, location: Point, layer: u32) {
    let entity = build_base_colonist(ecs, location, layer);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec![
                "Bracket Corp is going to save us?".to_string(),
                "No idea where the others are.".to_string(),
            ],
        },
    );
    commands.flush(ecs);
}
```

Now that the colonists' components are sorted out, let's move on to the activation system.

## Colonist Activation AI

The first thing to do is to change the criteria for which colonists wake up. We only want them to process a turn if they have the `Active` tag component attached to their entity, and if they are alive (we're keeping dead ones around for counting purposes). Open `src/game/colonists.rs` and change the system query to include `Active`. You also want to adjust the iterator to include a filter for living colonists:

```rust
        &mut ColonistStatus,
        &mut Position,
        &mut Dialog,
        &Active,
    )>::query();
    colonists
        .iter_mut(ecs)
        .filter(|(_, _, status, _, _, _)| **status == ColonistStatus::Alive)
        .for_each(|(entity, colonist, status, pos, dialog, _)| {
            let idx = map.get_layer(pos.layer as usize).point2d_to_index(pos.pt);

            // Check basics like "am I dead?"

            // Am I at the exit? If so, I can change my status to "rescued"
```

Now that colonists only function when activated, it's time to adjust the `Player` system to activate entities.

## Player System

Whenever the player sees an inactive entity, we want to check if it has a `CanBeActivated` tag. If it can, then we activate it. Open `src/game/player.rs`. Make the following changes (the new lines have a `+` next to them):

```rust
    if let Some(vt) = visible {
+        let mut commands = legion::systems::CommandBuffer::new(ecs);
        // Update colonist status
+        let mut can_be_activated = <(Entity, &CanBeActivated, &Position)>::query();
+        can_be_activated.for_each_mut(ecs, |(entity, _, pos)| {
            if pos.layer == map.current_layer as u32
                && vt.contains(&pos.pt)
                && DistanceAlg::Pythagoras.distance2d(player_pos, pos.pt) < 6.0
            {
+                commands.remove_component::<CanBeActivated>(*entity);
+                commands.add_component(*entity, Active {});
            }
        });
```

This builds a command buffer, and looks to see if a visible entity can be activated. If it can, we check that it is within 6 tiles and visible. If all of that is true, then we add an `Active` component to it.

Lastly for this section, we can update the heads-up display a bit.

## Update the HUD

We've changed how we're counting colonists, so replace `render_colonist_panel` in `src/render/colonist_panel.rs` as follows:

```rust
pub fn render_colonist_panel(ctx: &mut BTerm, ecs: &World, current_layer: usize) -> i32 {
    let mut query = <(Entity, &Colonist, &Position, &ColonistStatus)>::query();
    let mut total_colonists = 0;
    let mut colonists_on_layer = 0;
    let mut located_alive = 0;
    let mut located_dead = 0;
    let mut died_in_rescue = 0;
    let mut rescued = 0;

    query.for_each(ecs, |(entity, _, pos, status)| {
        total_colonists += 1;
        if pos.layer == current_layer as u32 && *status != ColonistStatus::Rescued {
            colonists_on_layer += 1;
        }
        if let Ok(entry) = ecs.entry_ref(*entity) {
            if let Ok(_) = entry.get_component::<Active>() {
                match *status {
                    ColonistStatus::Alive => located_alive += 1,
                    ColonistStatus::StartedDead => located_dead += 1,
                    ColonistStatus::DiedAfterStart => died_in_rescue += 1,
                    ColonistStatus::Rescued => rescued += 1,
                    _ => {}
                }
            }
        }
    });
```

There are still some problems with the HUD, and these will be resolved in later updates---it wasn't a big priority at the time, I just wanted a vague idea of what's going on. Ignoring this now caused a few heartaches later!

## Wrap-up

(Screenshot)

> You can find the source code for `active_components` [here](https://github.com/thebracket/secbot-2021-7drl/tree/tutorial/tutorial/active_components/).


This was a relatively small set of changes, but has smoothed over how entity activation works. Next up, we'll make colonists path properly across levels.

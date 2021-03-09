use crate::components::*;
use legion::systems::CommandBuffer;
use legion::*;

pub fn spawn_dialog(ecs: &mut World) {
    let mut commands = CommandBuffer::new(ecs);
    <(&mut Dialog, &Position, &Active)>::query()
        .iter_mut(ecs)
        .for_each(|(dialog, pos, _)| {
            if !dialog.lines.is_empty() {
                let line = dialog.lines[0].clone();
                dialog.lines.remove(0);
                commands.push((Speech { lifetime: 40 }, pos.clone(), Description(line)));
            }
        });
    commands.flush(ecs);
}

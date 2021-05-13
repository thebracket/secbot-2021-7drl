use legion::Entity;

pub struct Targeting {
    pub targets: Vec<(Entity, f32)>, // (entity / distance)
    pub current_target: Option<Entity>,
    pub index: usize,
}
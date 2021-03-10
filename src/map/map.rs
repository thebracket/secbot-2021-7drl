use super::{Layer, NUM_LAYERS};
use bracket_lib::prelude::*;
use legion::World;

pub struct Map {
    pub current_layer: usize,
    layers: Vec<Layer>,
}

impl Map {
    pub fn new(ecs: &mut World) -> Self {
        let mut layers = Vec::with_capacity(NUM_LAYERS);
        for i in 0..NUM_LAYERS {
            layers.push(Layer::new(i, ecs));
        }
        Self {
            current_layer: 1, // TODO: Set me back
            layers,
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        self.layers[self.current_layer].render(ctx);
    }

    pub fn get_current(&self) -> &Layer {
        &self.layers[self.current_layer]
    }

    pub fn get_current_mut(&mut self) -> &mut Layer {
        &mut self.layers[self.current_layer]
    }

    pub fn get_layer(&self, layer: usize) -> &Layer {
        &self.layers[layer]
    }

    #[allow(dead_code)]
    pub fn get_layer_mut(&mut self, layer: usize) -> &mut Layer {
        &mut self.layers[layer]
    }

    pub fn set_current_layer(&mut self, new_layer: usize) {
        self.current_layer = new_layer;
    }
}

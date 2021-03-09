use super::queries::ColonyInfo;
use crate::LAYER_TEXT;
use bracket_lib::prelude::*;

pub fn render_colony_info(batch: &mut DrawBatch, colony: &ColonyInfo) {
    batch.target(LAYER_TEXT); // Draw on the text layer
    batch.print_color(
        Point::new(82, 8),
        format!("Total Colonists   : {}", colony.total_colonists),
        ColorPair::new(LIME_GREEN, BLACK),
    );
    batch.print_color(
        Point::new(82, 9),
        format!("   (On this level): {}", colony.colonists_on_layer),
        ColorPair::new(GREEN, BLACK),
    );
    batch.print_color(
        Point::new(82, 10),
        format!(" (Located & Alive): {}", colony.located_alive),
        ColorPair::new(LIME_GREEN, BLACK),
    );
    batch.print_color(
        Point::new(82, 11),
        format!("  (Located & Dead): {}", colony.located_dead),
        ColorPair::new(HOT_PINK, BLACK),
    );
    batch.print_color(
        Point::new(82, 12),
        format!("  (Died in Rescue): {}", colony.died_in_rescue),
        ColorPair::new(RED, BLACK),
    );
    batch.print_color(
        Point::new(82, 13),
        format!("         (Rescued): {}", colony.rescued),
        ColorPair::new(LIME_GREEN, BLACK),
    );
}

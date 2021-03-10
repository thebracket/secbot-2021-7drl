use super::queries::TargetInfo;
use super::safe_print_color;
use crate::LAYER_TEXT;
use bracket_lib::prelude::*;

pub fn render_targeting(batch: &mut DrawBatch, target: &TargetInfo) {
    batch.target(LAYER_TEXT); // Draw on the text layer
    if let Some(_t) = &target.target {
        safe_print_color(
            batch,
            Point::new(82, 15),
            format!(
                "Target: {}",
                target.name.as_ref().unwrap_or(&"n/a".to_string())
            ),
            ColorPair::new(target.color.unwrap(), BLACK),
        );
        safe_print_color(
            batch,
            Point::new(82, 16),
            "[T] to cycle targets",
            ColorPair::new(GOLD, BLACK),
        );
        safe_print_color(
            batch,
            Point::new(82, 17),
            "[F] to fire weapons at target",
            ColorPair::new(GOLD, BLACK),
        );
    } else {
        safe_print_color(
            batch,
            Point::new(82, 15),
            "No current target",
            ColorPair::new(GRAY, BLACK),
        );
    }
}

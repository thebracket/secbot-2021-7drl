use super::queries::PlayerStatus;
use super::safe_print_color;
use crate::LAYER_TEXT;
use bracket_lib::prelude::*;

pub fn render_status(batch: &mut DrawBatch, status: &PlayerStatus) {
    batch.target(LAYER_TEXT); // Draw on the text layer
    batch.bar_horizontal(
        Point::new(82, 3),
        29,
        status.current_hp,
        status.max_hp,
        ColorPair::new(RED, DARK_RED),
    );
    batch.print_color_centered_at(
        Point::new(97, 3),
        format!("HP: {} / {}", status.current_hp, status.max_hp),
        ColorPair::new(WHITE, RED),
    );
    safe_print_color(
        batch,
        Point::new(82, 4),
        format!("Property Damage: {}", status.property_damage),
        ColorPair::new(GRAY, BLACK),
    );

    let (color, phrase) = if status.human_resources < 10 {
        (RED, "About to kill you")
    } else if status.human_resources < 20 {
        (ORANGE, "Cranky")
    } else if status.human_resources < 30 {
        (ORANGE, "Quite Concerned")
    } else if status.human_resources < 40 {
        (YELLOW, "Nervous")
    } else if status.human_resources < 60 {
        (GRAY, "Normal")
    } else if status.human_resources < 70 {
        (GREEN, "Content")
    } else if status.human_resources < 90 {
        (GREEN, "You're doing great!")
    } else {
        (GREEN, "Ecstatic")
    };

    safe_print_color(
        batch,
        Point::new(82, 5),
        "Human Resources Status:".to_string(),
        ColorPair::new(GRAY, BLACK),
    );
    safe_print_color(
        batch,
        Point::new(82, 6),
        phrase.to_string(),
        ColorPair::new(color, BLACK),
    );
}

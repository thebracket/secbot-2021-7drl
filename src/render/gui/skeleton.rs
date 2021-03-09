use crate::LAYER_TEXT;
use bracket_lib::prelude::*;

pub fn render_panels(batch: &mut DrawBatch) {
    batch.target(LAYER_TEXT); // Draw on the text layer
    batch.draw_box(
        Rect::with_exact(81, 0, 111, 30),
        ColorPair::new(DARK_GRAY, BLACK),
    );
    batch.print_color_centered_at(
        Point::new(97, 1),
        "SecBot - 2021 7DRL",
        ColorPair::new(WHITE, BLACK),
    );
}

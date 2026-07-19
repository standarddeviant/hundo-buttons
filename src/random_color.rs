use egui::{Color32, RichText};
use rand::Rng;

pub fn random_pastel_color() -> Color32 {
    // Examples on how to use
    // Display text using the pastel color
    // ui.label(
    //     RichText::new("Soft pastel color text!")
    //         .color(self.random_color)
    //         .size(20.0),
    // );
    //
    // Draw a rounded swatch card of the color
    // let (_, rect) = ui.allocate_space(egui::vec2(120.0, 60.0));
    //         ui.painter().rect_filled(rect, 8.0, self.random_color);
    //     });
    // }

    // Channels are heavily biased toward full brightness (255)
    Color32::from_rgb(
        rand::random_range(175..=255),
        rand::random_range(175..=255),
        rand::random_range(175..=255),
    )
}

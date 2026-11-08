
 //! Math / LaTeX panel for mathematical equations.
 //!
 //! First stage: verify the native EffectCraft docking integration.

 use crate::EffectcraftApp;
 use egui::{Align2, FontId, Rect, Ui};

 /// Draw the Math / LaTeX panel.
 pub fn show(_app: &mut EffectcraftApp, ui: &mut Ui, rect: Rect) {
     ui.painter().text(
         rect.center(),
         Align2::CENTER_CENTER,
         "Math / LaTeX - Panel ready!",
         FontId::proportional(16.0),
         ui.visuals().text_color(),
     );
 }

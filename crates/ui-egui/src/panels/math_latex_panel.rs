
//! Math / LaTeX panel for mathematical equations.
//!
//! Current stage:
//! - editable LaTeX source
//! - render preview button
//! - PNG preview shown as an egui texture
//!
//! Future work:
//! - store equations in the project
//! - insert rendered output into a composition/layer

// use egui::{Id, Rect, TextEdit, TextureHandle, TextureOptions, Ui};
use egui::{Rect, TextEdit, TextureHandle, TextureOptions, Ui};
use latex_rust::{latex_to_png, MathFont, PngOptions};

use crate::EffectcraftApp; 

fn render_latex_texture(
    ctx: &egui::Context,
    latex_src: &str,
) -> Result<TextureHandle, String> {
    let font = MathFont::stix_two_math()
        .map_err(|e| format!("Font error: {e}"))?;

    // let png_bytes = latex_to_png(latex_src, &font, &PngOptions::new())
    //     .map_err(|e| format!("LaTeX render error: {e}"))?;
    // Render the equation in white for EffectCraft's dark theme. 
    let colored_latex = format!(r"{{\color{{white}} {latex_src}}}");

    let png_bytes = latex_to_png(&colored_latex, &font, &PngOptions::new())
        .map_err(|e| format!("LaTeX render error: {e}"))?;

    let img = image::load_from_memory(&png_bytes)
        .map_err(|e| format!("PNG decode error: {e}"))?
        .to_rgba8();

    let size = [img.width() as usize, img.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        size,
        img.as_raw(),
    );

    Ok(ctx.load_texture(
        "math_latex_preview",
        color_image,
        TextureOptions::LINEAR,
    ))
}

pub(crate) fn show(_app: &mut EffectcraftApp, ui: &mut Ui, _rect: Rect) {
    const SOURCE_ID: &str = "math_latex/source";
    const ERROR_ID: &str = "math_latex/error";
    const TEXTURE_ID: &str = "math_latex/texture";

    // let ctx = ui.ctx().clone();
    // Stable identifiers for the editor and preview.
    let source_id = ui.make_persistent_id(SOURCE_ID);
    let error_id = ui.make_persistent_id(ERROR_ID);
    let texture_id = ui.make_persistent_id(TEXTURE_ID);

    let mut source = ui.ctx().data_mut(|d| {
        d.get_persisted::<String>(source_id)
            .unwrap_or_else(|| String::from(
                r"\pi^{+}\rightarrow\mu^{+}+\nu_{\mu}"
            ))
    });

    ui.heading("Math / LaTeX");
    ui.label("Escribe una expresión matemática en LaTeX y genera una vista previa.");

    ui.add(
        TextEdit::multiline(&mut source)
            .desired_rows(4)
            .hint_text(r"\frac{1}{2}, \pi^\pm, e^- + e^+ \to \mu^- + \mu^+"),
    );

    ui.horizontal(|ui| {
        if ui.button("Render preview").clicked() {
            match render_latex_texture(&ui.ctx(), &source) {
                Ok(texture) => {
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(texture_id, texture);
                        d.remove::<String>(error_id);
                    });
                }
                Err(err) => {
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(error_id, err);
                    });
                }
            }
        }
    });

    ui.ctx().data_mut(|d| {
        d.insert_persisted(source_id, source.clone());
    });

    if let Some(err) = ui.ctx().data_mut(|d| d.get_temp::<String>(error_id)) {
        ui.separator();
        ui.colored_label(egui::Color32::RED, err);
    }

    ui.separator();
    ui.label("Vista previa:");

    if let Some(texture) = ui.ctx().data_mut(|d| d.get_temp::<TextureHandle>(texture_id)) {
        let available_width = ui.available_width().max(200.0);
        let mut size = texture.size_vec2();

        if size.x > available_width {
            let scale = available_width / size.x;
            size *= scale;
        }

        ui.image((texture.id(), size));
    } else {
        ui.weak("Aún no hay vista previa. Pulsa \"Render preview\".");
    }
}
 

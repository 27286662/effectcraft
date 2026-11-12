
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


// New line from 21 too 29 
// fn render_latex_png(latex_src: &str) -> Result<Vec<u8>, String> {
//     let font = MathFont::stix_two_math()
//         .map_err(|e| format!("Font error: {e}"))?;

//     let colored_latex = format!(r"{{\color{{white}} {latex_src}}}");


//     let png_bytes = render_latex_png(&colored_latex)?; // replace the lines below // error
//     // latex_to_png(&colored_latex, &font, &PngOptions::new())
//     //     .map_err(|e| format!("LaTeX render error: {e}"))
// }

// lines from 34 too 44 worked now we need to improve the image
// fn render_latex_png(latex_src: &str) -> Result<Vec<u8>, String> {
//     let font = MathFont::stix_two_math()
//         .map_err(|e| format!("Font error: {e}"))?;

//     // Render the equation in white.
//     let colored_latex = format!(r"{{\color{{black}} {latex_src}}}");

//     // Generate and return the PNG bytes.
//     latex_to_png(&colored_latex, &font, &PngOptions::new())
//         .map_err(|e| format!("LaTeX render error: {e}"))
// }

// new lines from 47 to 65 improvment on the render image
// fn render_latex_png(latex_src: &str) -> Result<Vec<u8>, String> {
// fn render_latex_png(latex_src: &str, color: &str) -> Result<Vec<u8>, String> {
    fn render_latex_png(latex_src: &str, color: &str, font_size_pt: i64, ) -> Result<Vec<u8>, String> {
    use latex_rust::Dim;

    let font = MathFont::stix_two_math()
        .map_err(|e| format!("Font error: {e}"))?;

    // Black for the composition background.
    // let colored_latex = format!(r"{{\color{{black}} {latex_src}}}");
    // Apply the color selected in the Math / LaTeX panel.
    let colored_latex = format!(r"{{\color{{{color}}} {latex_src}}}");

    let mut opts = PngOptions::new();

    // Much better quality for imported layer:
    // opts.font_size_pt = Dim::from_i64(48);  // try 48 first
    opts.font_size_pt = Dim::from_i64(font_size_pt);
    opts.dpi = Dim::from_i64(400);          // try 300–600
    opts.display = true;                    // nicer for standalone equations

    latex_to_png(&colored_latex, &font, &opts)
        .map_err(|e| format!("LaTeX render error: {e}"))
}
    

// from 32 too 110
// fn insert_latex_layer(app: &mut EffectcraftApp, ctx: &egui::Context, latex_src: &str, color: &str, -> Result<(), String> {
fn insert_latex_layer(
    app: &mut EffectcraftApp,
    ctx: &egui::Context,
    latex_src: &str,
    color: &str,
    font_size_pt: i64,
) -> Result<(), String> {

    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    // A composition must be open.
    if app.session.active_comp_id().is_none() {
        return Err("Open a composition first.".into());
    }

    // Require a saved project so the PNG has a stable location.
    let project_file = app.session.path.as_deref()
        .ok_or("Save the EffectCraft project before inserting equations.")?;

    let project_path = Path::new(project_file);

    let project_dir = project_path.parent()
        .ok_or("Cannot determine the project directory.")?;

    let project_name = project_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("effectcraft");

    let assets_dir = project_dir
        .join(format!("{project_name}_assets"))
        .join("latex");

    std::fs::create_dir_all(&assets_dir)
        .map_err(|e| format!("Cannot create assets directory: {e}"))?;

    // Render the current equation.
    // let png_bytes = render_latex_png(latex_src)?;
    // let png_bytes = render_latex_png(latex_src, color)?;
    let png_bytes = render_latex_png(latex_src, color, font_size_pt)?;

    // Unique filename: each insertion remains independent.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();

    let png_path = assets_dir.join(format!("equation-{timestamp}.png"));

    std::fs::write(&png_path, png_bytes)
        .map_err(|e| format!("Cannot save equation PNG: {e}"))?;

    let png_path = std::fs::canonicalize(&png_path)
        .map_err(|e| format!("Cannot resolve PNG path: {e}"))?;

    // Import the image and add it to the active composition.
    let result = crate::menus::invoke(
        app,
        ctx,
        "file.import",
        serde_json::json!({
            "paths": [png_path.to_string_lossy().to_string()],
            "sequence": false,
            "addToComp": true,
            "background": false
        }),
    )?;

    // File imports may return per-file errors without failing the command.
    if let Some(errors) = result.get("errors").and_then(|v| v.as_array()) {
        if !errors.is_empty() {
            return Err(format!("Import errors: {errors:?}"));
        }
    }

    if result.get("items")
        .and_then(|v| v.as_array())
        .is_none_or(|items| items.is_empty())
    {
        return Err("No image was imported.".into());
    }

    Ok(())
}

// fn render_latex_texture(
//     ctx: &egui::Context,
//     latex_src: &str,
// ) -> Result<TextureHandle, String> {
    fn render_latex_texture(
        ctx: &egui::Context,
        latex_src: &str,
        color: &str,
    ) -> Result<TextureHandle, String> {
    let font = MathFont::stix_two_math()
        .map_err(|e| format!("Font error: {e}"))?;

    // let png_bytes = latex_to_png(latex_src, &font, &PngOptions::new())
    //     .map_err(|e| format!("LaTeX render error: {e}"))?;
    // Render the equation in white for EffectCraft's dark theme. 
    let colored_latex = format!(r"{{\color{{{color}}} {latex_src}}}"); // let colored_latex = format!(r"{{\color{{white}} {latex_src}}}");

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

pub(crate) fn show(app: &mut EffectcraftApp, ui: &mut Ui, _rect: Rect) // _app -> app
    {
    const SOURCE_ID: &str = "math_latex/source";
    const ERROR_ID: &str = "math_latex/error";
    const TEXTURE_ID: &str = "math_latex/texture";

    // Equation size, remembered across UI frames.
    let font_size_id = ui.make_persistent_id("math_latex/font_size");

    let mut font_size_pt = ui.ctx().data_mut(|data| {
        data.get_persisted::<i64>(font_size_id)
    }).unwrap_or(32);

    // Store the selected equation color.
    let layer_color_id = ui.make_persistent_id("math_latex/layer_color");

    let mut layer_color = ui.ctx().data_mut(|data| {
        data.get_persisted::<String>(layer_color_id)
    }).unwrap_or_else(|| "black".to_string());

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

    // new lines from 222 too 237 
    // Equation color selector.
    // ui.horizontal(|ui| {
    //     ui.label("Layer color:");

    //     ui.add(
    //         egui::Slider::new(&mut font_size_pt, 24..=96)
    //         .text("Equation size (pt)")
    //     );

    //     ui.radio_value(
    //         &mut layer_color,
    //         "black".to_string(),
    //         "Black",
    //     );

    //     ui.radio_value(
    //         &mut layer_color,
    //         "white".to_string(),
    //         "White",
    //     );
    // });
    // Equation color selector.
    ui.horizontal(|ui| {
        ui.label("Layer color:");

        ui.radio_value(
            &mut layer_color,
            "black".to_string(),
            "Black",
        );

        ui.radio_value(
            &mut layer_color,
            "white".to_string(),
            "White",
        );
    });

    // Equation size selector, displayed on a separate row.
    ui.add(
        egui::Slider::new(&mut font_size_pt, 16..=96)
            .text("Equation size (pt)")
    );

    ui.horizontal(|ui| {
        if ui.button("Render preview").clicked() {
            // match render_latex_texture(&ui.ctx(), &source) {
            match render_latex_texture(ui.ctx(), &source, &layer_color) {
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

    // from 191 too 204
    if ui.button("Insert into composition").clicked() {
    // match insert_latex_layer(app, ui.ctx(), &source) {
    match insert_latex_layer(app, ui.ctx(), &source, &layer_color, font_size_pt,) {
        Ok(()) => {
            ui.ctx().data_mut(|d| {
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

    // ui.ctx().data_mut(|d| {
    //     d.insert_persisted(source_id, source.clone());
    // });
    // added color almacenamient
    ui.ctx().data_mut(|d| {
    d.insert_persisted(source_id, source.clone());
    d.insert_persisted(layer_color_id, layer_color.clone());
    d.insert_persisted(font_size_id, font_size_pt);
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

        // ui.image((texture.id(), size));

        // Choose a contrasting background for the equation preview.
        let preview_bg = if layer_color == "black" {
            egui::Color32::from_rgb(245, 245, 245)
        } else {
            egui::Color32::from_rgb(35, 35, 35)
        };

        egui::Frame::default() 
            .fill(preview_bg)
            .show(ui, |ui| {
                ui.image((texture.id(), size));
            });

    } else {
        ui.weak("Aún no hay vista previa. Pulsa \"Render preview\".");
    }
}
 

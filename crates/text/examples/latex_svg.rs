
use latex_rust::{latex_to_svg, MathFont, SvgOptions};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Fuente matemática
    let font = MathFont::stix_two_math()?;

    // Decaimiento del pion
    let equation = r"\pi^{+}\rightarrow\mu^{+}+\nu_{\mu}";

    // Generar SVG vectorial
    let svg = latex_to_svg(equation, &font, &SvgOptions::new())?;

    // Crear directorio de salida
    fs::create_dir_all("latex-output")?;

    // Guardar ecuacion
    let output = "latex-output/pion_decay.svg";
    fs::write(output, svg)?;

    println!("Ecuacion: {equation}");
    println!("SVG generado en: {output}");

    Ok(())
}

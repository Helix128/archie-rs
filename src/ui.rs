use colored::Colorize;
use colored::Color;
pub fn fill_bar(pct: f64, width: usize, filled_color: Color, empty_color: Color) {
    let filled_length = (pct / 100.0 * width as f64).round() as usize;
    let empty_length = width - filled_length;
    let result = format!(
        "[{}{}]",
        "▱".repeat(filled_length).color(filled_color),
        "▰".repeat(empty_length).color(empty_color)
    );

    println!("{}", result);
}

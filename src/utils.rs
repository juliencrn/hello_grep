use ansi_term::Colour;

pub fn colorize(color: Colour, text: &str, display_color: bool) -> String {
    if display_color {
        format!("{}", color.paint(text))
    } else {
        text.to_string()
    }
}

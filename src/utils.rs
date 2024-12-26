/// Formats a string to a fixed width for the i2c, 16x2 LCD display.
pub fn format_string(input: &str, width: usize) -> String {
    format!("{: <width$}", input, width = width)
}

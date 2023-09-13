use colored::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn cycle_color(text: &str, start: usize) -> String {
    let colors = ["red", "yellow", "green", "cyan", "blue", "magenta"];
    let mut colored_text = String::new();

    for (i, c) in text.chars().enumerate() {
        let color = colors[(start + i) % colors.len()];
        colored_text.push_str(&c.to_string().color(color).to_string());
    }

    colored_text
}

fn main() -> color_eyre::Result<()> {
    let path = Path::new("names.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut start_color = 0;
    for line in reader.lines() {
        let name = line?;
        println!("{}", cycle_color(&name, start_color));
        start_color = (start_color + 1) % 6; // Cycle through 6 colors
    }

    Ok(())
}

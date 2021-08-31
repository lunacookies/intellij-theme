mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut intellij_theme = ThemeBuilder::new("IntelliJ".to_string(), Type::Dark);
    imp::add_rules(&mut intellij_theme, &palette);
    intellij_theme.build().save()?;

    Ok(())
}

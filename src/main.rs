mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut builder = ThemeBuilder::new("Darcula".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &palette);
    imp::add_rules_last(&mut builder);
    builder.build().save()?;

    let mut builder = ThemeBuilder::new("Darcula GoLand".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &palette);
    imp::add_rules_goland(&mut builder, &palette);
    imp::add_rules_last(&mut builder);
    builder.build().save()?;

    Ok(())
}

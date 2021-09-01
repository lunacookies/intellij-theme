mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use palette::Palette;
use std::io;

fn main() -> io::Result<()> {
    let mut builder = ThemeBuilder::new("Darcula IntelliJ".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &Palette::Normal);
    imp::add_rules_intellij(&mut builder, &Palette::Normal);
    imp::add_rules_last(&mut builder, &Palette::Normal);
    builder.build().save()?;

    let mut builder = ThemeBuilder::new("Darcula GoLand".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &Palette::Normal);
    imp::add_rules_goland(&mut builder, &Palette::Normal);
    imp::add_rules_last(&mut builder, &Palette::Normal);
    builder.build().save()?;

    let mut builder = ThemeBuilder::new("Darcula CLion".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &Palette::Normal);
    imp::add_rules_clion(&mut builder, &Palette::Normal);
    imp::add_rules_last(&mut builder, &Palette::Normal);
    builder.build().save()?;

    let mut builder = ThemeBuilder::new("Darcula AppCode".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &Palette::Normal);
    imp::add_rules_appcode(&mut builder, &Palette::Normal);
    imp::add_rules_last(&mut builder, &Palette::Normal);
    builder.build().save()?;

    let mut builder = ThemeBuilder::new("Rider".to_string(), Type::Dark);
    imp::workspace_colors(&mut builder, &Palette::GreyscaleBase);
    imp::add_rules_rider(&mut builder, &Palette::GreyscaleBase);
    imp::add_rules_last(&mut builder, &Palette::GreyscaleBase);
    builder.build().save()?;

    Ok(())
}
